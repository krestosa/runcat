    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn filetime_u64(value: FileTime) -> u64 {
        ((value.high as u64) << 32) | value.low as u64
    }

    fn read_cpu_times() -> Option<(u64, u64, u64)> {
        unsafe {
            let mut idle: FileTime = zeroed();
            let mut kernel: FileTime = zeroed();
            let mut user: FileTime = zeroed();
            if GetSystemTimes(&mut idle, &mut kernel, &mut user) == 0 {
                return None;
            }
            Some((filetime_u64(idle), filetime_u64(kernel), filetime_u64(user)))
        }
    }

    fn cpu_usage_and_store(state: &mut AppState) -> Option<f64> {
        let (idle, kernel, user) = read_cpu_times()?;
        if state.last_kernel == 0 && state.last_user == 0 {
            state.last_idle = idle;
            state.last_kernel = kernel;
            state.last_user = user;
            return None;
        }

        let idle_delta = idle.saturating_sub(state.last_idle);
        let kernel_delta = kernel.saturating_sub(state.last_kernel);
        let user_delta = user.saturating_sub(state.last_user);

        state.last_idle = idle;
        state.last_kernel = kernel;
        state.last_user = user;

        let total = kernel_delta.saturating_add(user_delta);
        if total == 0 {
            return None;
        }

        let busy = total.saturating_sub(idle_delta);
        Some((busy as f64 * 100.0 / total as f64).clamp(0.0, 100.0))
    }

    fn target_frame_interval(cpu_percent: f64, settings: Settings) -> f64 {
        let mut utilization = (cpu_percent / 100.0).clamp(0.0, 1.0);
        if settings.invert_speed {
            utilization = 1.0 - utilization;
        }

        // Same feel as current GNOME RunCat, independently implemented:
        // one full five-frame cycle ranges from about 1100 ms at idle to 250 ms at full CPU.
        let cycle_ms = 250.0 + 850.0 * (1.0 - utilization).powi(2);
        (cycle_ms / FRAME_COUNT as f64 / settings.speed_multiplier).clamp(10.0, 2000.0)
    }

    fn should_idle(cpu_percent: f64, settings: Settings) -> bool {
        !settings.invert_speed && cpu_percent <= settings.idle_threshold
    }

    unsafe fn release_stream(stream: *mut Unknown) {
        if !stream.is_null() {
            ((*(*stream).vtbl).release)(stream);
        }
    }

    fn load_frame_pixels(bytes: &[u8]) -> Result<FramePixels, &'static str> {
        if bytes.len() > u32::MAX as usize {
            return Err("asset is too large");
        }

        unsafe {
            let stream = SHCreateMemStream(bytes.as_ptr(), bytes.len() as Uint);
            if stream.is_null() {
                return Err("SHCreateMemStream failed");
            }

            let mut image: *mut c_void = null_mut();
            if GdipLoadImageFromStream(stream, &mut image) != 0 || image.is_null() {
                release_stream(stream);
                return Err("GdipLoadImageFromStream failed");
            }

            let mut width = 0;
            let mut height = 0;
            if GdipGetImageWidth(image, &mut width) != 0
                || GdipGetImageHeight(image, &mut height) != 0
                || width == 0
                || height == 0
            {
                GdipDisposeImage(image);
                release_stream(stream);
                return Err("invalid cat frame dimensions");
            }

            let rect = Rect {
                x: 0,
                y: 0,
                width: width as i32,
                height: height as i32,
            };
            let mut locked: BitmapData = zeroed();
            if GdipBitmapLockBits(
                image,
                &rect,
                IMAGE_LOCK_MODE_READ,
                PIXEL_FORMAT_32BPP_ARGB,
                &mut locked,
            ) != 0
                || locked.scan0.is_null()
            {
                GdipDisposeImage(image);
                release_stream(stream);
                return Err("GdipBitmapLockBits failed");
            }

            let row_bytes = width as usize * 4;
            let mut pixels = vec![0u8; row_bytes * height as usize];
            for y in 0..height as usize {
                let source = (locked.scan0 as *const u8).offset(y as isize * locked.stride as isize);
                let destination = pixels.as_mut_ptr().add(y * row_bytes);
                copy_nonoverlapping(source, destination, row_bytes);
            }

            GdipBitmapUnlockBits(image, &mut locked);
            GdipDisposeImage(image);
            release_stream(stream);

            Ok(FramePixels {
                width,
                height,
                pixels,
            })
        }
    }

    fn alpha_bounds(frame: &FramePixels) -> AlphaBounds {
        let mut min_x = frame.width;
        let mut min_y = frame.height;
        let mut max_x = 0u32;
        let mut max_y = 0u32;
        let mut found = false;

        for y in 0..frame.height {
            for x in 0..frame.width {
                let index = ((y * frame.width + x) * 4) as usize;
                if frame.pixels[index + 3] != 0 {
                    found = true;
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        if !found {
            return AlphaBounds {
                x: 0,
                y: 0,
                width: frame.width.max(1),
                height: frame.height.max(1),
            };
        }

        AlphaBounds {
            x: min_x,
            y: min_y,
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        }
    }

    fn union_alpha_bounds(frames: &[FramePixels]) -> AlphaBounds {
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0u32;
        let mut max_y = 0u32;

        for frame in frames {
            let bounds = alpha_bounds(frame);
            min_x = min_x.min(bounds.x);
            min_y = min_y.min(bounds.y);
            max_x = max_x.max(bounds.x + bounds.width - 1);
            max_y = max_y.max(bounds.y + bounds.height - 1);
        }

        AlphaBounds {
            x: min_x,
            y: min_y,
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        }
    }
