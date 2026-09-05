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
                let source =
                    (locked.scan0 as *const u8).offset(y as isize * locked.stride as isize);
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

    fn create_icon(
        frame: &FramePixels,
        crop: AlphaBounds,
        light_theme: bool,
        art_size: u32,
        canvas_size: u32,
    ) -> Result<Hicon, &'static str> {
        let canvas_size = canvas_size.clamp(MIN_CAT_SIZE, MAX_CAT_SIZE);
        let target = art_size.clamp(MIN_CAT_SIZE, canvas_size);
        let crop_width = crop.width.max(1);
        let crop_height = crop.height.max(1);

        let (draw_width, draw_height) = if crop_width >= crop_height {
            let height = ((target as u64 * crop_height as u64 + crop_width as u64 / 2)
                / crop_width as u64) as u32;
            (target, height.max(1))
        } else {
            let width = ((target as u64 * crop_width as u64 + crop_height as u64 / 2)
                / crop_height as u64) as u32;
            (width.max(1), target)
        };

        let offset_x = (canvas_size - draw_width) / 2;
        let offset_y = (canvas_size - draw_height) / 2;
        let cat_channel = if light_theme { 0u8 } else { 255u8 };
        let mut canvas = vec![0u8; (canvas_size * canvas_size * 4) as usize];

        for y in 0..draw_height {
            let source_y = crop.y
                + ((y as u64 * crop_height as u64) / draw_height as u64)
                    .min(crop_height.saturating_sub(1) as u64) as u32;
            for x in 0..draw_width {
                let source_x = crop.x
                    + ((x as u64 * crop_width as u64) / draw_width as u64)
                        .min(crop_width.saturating_sub(1) as u64) as u32;
                let source_index = ((source_y * frame.width + source_x) * 4) as usize;
                let destination_x = x + offset_x;
                let destination_y = y + offset_y;
                let destination_index =
                    ((destination_y * canvas_size + destination_x) * 4) as usize;
                let alpha = frame.pixels[source_index + 3];

                if alpha != 0 {
                    canvas[destination_index] = cat_channel;
                    canvas[destination_index + 1] = cat_channel;
                    canvas[destination_index + 2] = cat_channel;
                    canvas[destination_index + 3] = alpha;
                }
            }
        }

        unsafe {
            let info = BitmapInfo {
                header: BitmapInfoHeader {
                    size: size_of::<BitmapInfoHeader>() as Dword,
                    width: canvas_size as i32,
                    height: -(canvas_size as i32),
                    planes: 1,
                    bit_count: 32,
                    compression: BI_RGB,
                    size_image: 0,
                    x_pels_per_meter: 0,
                    y_pels_per_meter: 0,
                    clr_used: 0,
                    clr_important: 0,
                },
                colors: [RgbQuad {
                    blue: 0,
                    green: 0,
                    red: 0,
                    reserved: 0,
                }],
            };

            let mut dib_bits: *mut c_void = null_mut();
            let color_bitmap =
                CreateDIBSection(0, &info, DIB_RGB_COLORS, &mut dib_bits, 0, 0);
            if color_bitmap == 0 || dib_bits.is_null() {
                return Err("CreateDIBSection failed");
            }

            copy_nonoverlapping(canvas.as_ptr(), dib_bits as *mut u8, canvas.len());

            let mask_bitmap =
                CreateBitmap(canvas_size as i32, canvas_size as i32, 1, 1, null());
            if mask_bitmap == 0 {
                DeleteObject(color_bitmap);
                return Err("CreateBitmap failed");
            }

            let icon_info = IconInfo {
                is_icon: 1,
                x_hotspot: 0,
                y_hotspot: 0,
                mask: mask_bitmap,
                color: color_bitmap,
            };
            let icon = CreateIconIndirect(&icon_info);
            DeleteObject(mask_bitmap);
            DeleteObject(color_bitmap);

            if icon == 0 {
                return Err("CreateIconIndirect failed");
            }
            Ok(icon)
        }
    }

    fn build_icon_array(
        frames: &[FramePixels],
        crop: AlphaBounds,
        light_theme: bool,
        art_size: u32,
        canvas_size: u32,
    ) -> Result<[Hicon; FRAME_COUNT], &'static str> {
        if frames.len() != FRAME_COUNT {
            return Err("expected exactly five running cat frames");
        }

        let mut icons = [0; FRAME_COUNT];
        for (index, frame) in frames.iter().enumerate() {
            match create_icon(frame, crop, light_theme, art_size, canvas_size) {
                Ok(icon) => icons[index] = icon,
                Err(error) => {
                    destroy_icon_array(&icons);
                    return Err(error);
                }
            }
        }
        Ok(icons)
    }

    fn build_visuals(
        frames: &[FramePixels],
        sleep: &FramePixels,
        light_theme: bool,
        settings: Settings,
    ) -> Result<IconSet, &'static str> {
        if frames.len() != FRAME_COUNT {
            return Err("expected exactly five running cat frames");
        }

        let width = frames[0].width;
        let height = frames[0].height;
        if frames
            .iter()
            .any(|frame| frame.width != width || frame.height != height)
        {
            return Err("running cat frame dimensions do not match");
        }

        let running_crop = union_alpha_bounds(frames);
        let sleeping_crop = alpha_bounds(sleep);
        let tray_size = settings.size_px.min(TRAY_CANVAS);
        let tray = build_icon_array(
            frames,
            running_crop,
            light_theme,
            tray_size,
            TRAY_CANVAS,
        )?;
        let tray_sleep = match create_icon(
            sleep,
            sleeping_crop,
            light_theme,
            tray_size,
            TRAY_CANVAS,
        ) {
            Ok(icon) => icon,
            Err(error) => {
                destroy_icon_array(&tray);
                return Err(error);
            }
        };

        let mut visuals = IconSet {
            tray,
            tray_sleep,
            overlay: [0; FRAME_COUNT],
            overlay_sleep: 0,
        };

        if settings.overlay_mode && settings.size_px > TRAY_CANVAS {
            let size = settings.size_px.clamp(TRAY_CANVAS + 1, MAX_CAT_SIZE);
            visuals.overlay =
                match build_icon_array(frames, running_crop, light_theme, size, size) {
                    Ok(icons) => icons,
                    Err(error) => {
                        destroy_visuals(&visuals);
                        return Err(error);
                    }
                };
            visuals.overlay_sleep =
                match create_icon(sleep, sleeping_crop, light_theme, size, size) {
                    Ok(icon) => icon,
                    Err(error) => {
                        destroy_visuals(&visuals);
                        return Err(error);
                    }
                };
        }

        Ok(visuals)
    }

    fn destroy_icon_array(icons: &[Hicon; FRAME_COUNT]) {
        unsafe {
            for icon in icons.iter().copied().filter(|icon| *icon != 0) {
                DestroyIcon(icon);
            }
        }
    }

    fn destroy_visuals(visuals: &IconSet) {
        destroy_icon_array(&visuals.tray);
        destroy_icon_array(&visuals.overlay);
        unsafe {
            if visuals.tray_sleep != 0 {
                DestroyIcon(visuals.tray_sleep);
            }
            if visuals.overlay_sleep != 0 {
                DestroyIcon(visuals.overlay_sleep);
            }
        }
    }

    fn current_tray_icon(state: &AppState) -> Hicon {
        if (state.is_idle || state.battery_paused || state.settings.manual_pause)
            && state.settings.sleep_idle
        {
            state.visuals.tray_sleep
        } else {
            state.visuals.tray[state.frame]
        }
    }

    fn current_overlay_icon(state: &AppState) -> Hicon {
        if state.visuals.overlay_sleep == 0 {
            return 0;
        }
        if (state.is_idle || state.battery_paused || state.settings.manual_pause)
            && state.settings.sleep_idle
        {
            state.visuals.overlay_sleep
        } else {
            state.visuals.overlay[state.frame]
        }
    }

    fn notify_data(hwnd: Hwnd, icon: Hicon, tooltip: &str) -> NotifyIconDataW {
        let mut data: NotifyIconDataW = unsafe { zeroed() };
        data.cb_size = size_of::<NotifyIconDataW>() as Dword;
        data.hwnd = hwnd;
        data.id = 1;
        data.flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        data.callback_message = TRAY_CALLBACK;
        data.icon = icon;
        set_notify_tip(&mut data, tooltip);
        data
    }

    fn set_notify_tip(data: &mut NotifyIconDataW, text: &str) {
        let label = wide(text);
        let copy_len = label.len().saturating_sub(1).min(data.tip.len() - 1);
        data.tip.fill(0);
        data.tip[..copy_len].copy_from_slice(&label[..copy_len]);
    }

    fn state_label(state: &AppState) -> &'static str {
        if state.settings.manual_pause {
            "Paused"
        } else if state.battery_paused {
            "Battery pause"
        } else if state.is_idle && state.settings.sleep_idle {
            "Sleeping"
        } else if state.is_idle {
            "Idle"
        } else {
            "Running"
        }
    }

    fn tray_tooltip(state: &AppState) -> String {
        let mut parts = Vec::with_capacity(4);
        if state.settings.tooltip_cpu {
            parts.push(format!("CPU {:.0}%", state.cpu_percent));
        }
        if state.settings.tooltip_ram {
            parts.push(format!("RAM {:.0}%", state.ram_percent));
        }
        if state.settings.tooltip_battery {
            let power = if state.power.on_battery {
                match state.power.battery_percent {
                    Some(percent) => format!("Battery {percent}%"),
                    None => "Battery".to_string(),
                }
            } else {
                "AC".to_string()
            };
            parts.push(power);
        }
        if state.settings.manual_pause || state.battery_paused || state.is_idle {
            parts.push(state_label(state).to_string());
        }

        if parts.is_empty() {
            "CatCPU".to_string()
        } else {
            format!("CatCPU — {}", parts.join(" · "))
        }
    }

    fn add_tray_icon(state: &mut AppState) -> bool {
        let icon = current_tray_icon(state);
        let tooltip = tray_tooltip(state);
        let mut data = notify_data(state.hwnd, icon, &tooltip);
        let added = unsafe { Shell_NotifyIconW(NIM_ADD, &mut data) != 0 };
        if added {
            state.last_tray_icon = icon;
            state.last_tooltip = tooltip;
        }
        added
    }

    fn update_tray_icon_if_changed(state: &mut AppState, force: bool) {
        let icon = current_tray_icon(state);
        if !force && state.last_tray_icon == icon {
            return;
        }
        let mut data = notify_data(state.hwnd, icon, "");
        data.flags = NIF_ICON;
        if unsafe { Shell_NotifyIconW(NIM_MODIFY, &mut data) != 0 } {
            state.last_tray_icon = icon;
        }
    }

    fn update_tray_tooltip_if_changed(state: &mut AppState, force: bool) {
        let tooltip = tray_tooltip(state);
        if !force && state.last_tooltip == tooltip {
            return;
        }
        let mut data = notify_data(state.hwnd, current_tray_icon(state), &tooltip);
        data.flags = NIF_TIP;
        if unsafe { Shell_NotifyIconW(NIM_MODIFY, &mut data) != 0 } {
            state.last_tooltip = tooltip;
        }
    }

    fn remove_tray_icon(hwnd: Hwnd) {
        let mut data = notify_data(hwnd, 0, "");
        data.flags = 0;
        unsafe {
            Shell_NotifyIconW(NIM_DELETE, &mut data);
        }
    }

    fn sync_overlay(state: &AppState) {
        if state.overlay_hwnd == 0 {
            return;
        }

        unsafe {
            if !state.settings.overlay_mode || state.settings.size_px <= TRAY_CANVAS {
                ShowWindow(state.overlay_hwnd, SW_HIDE);
                return;
            }

            let icon = current_overlay_icon(state);
            if icon == 0 {
                ShowWindow(state.overlay_hwnd, SW_HIDE);
                return;
            }

            let size = state
                .settings
                .size_px
                .clamp(TRAY_CANVAS + 1, MAX_CAT_SIZE) as i32;
            let mut work: WorkRect = zeroed();
            if SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                &mut work as *mut WorkRect as *mut c_void,
                0,
            ) == 0
            {
                return;
            }

            let x = work.right - size - 8;
            let y = work.bottom - size - 8;
            SetWindowPos(
                state.overlay_hwnd,
                HWND_TOPMOST,
                x,
                y,
                size,
                size,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
            SetLayeredWindowAttributes(
                state.overlay_hwnd,
                OVERLAY_COLOR_KEY,
                255,
                LWA_COLORKEY,
            );

            let dc = GetDC(state.overlay_hwnd);
            if dc == 0 {
                return;
            }
            let brush = CreateSolidBrush(OVERLAY_COLOR_KEY);
            if brush != 0 {
                let rect = WorkRect {
                    left: 0,
                    top: 0,
                    right: size,
                    bottom: size,
                };
                FillRect(dc, &rect, brush);
                DeleteObject(brush);
            }
            DrawIconEx(dc, 0, 0, icon, size, size, 0, 0, DI_NORMAL);
            ReleaseDC(state.overlay_hwnd, dc);
        }
    }

    unsafe extern "system" fn overlay_wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    fn rebuild_visuals(state: &mut AppState, force: bool) -> Result<(), &'static str> {
        let next_light = effective_light_theme(state.settings);
        let theme_changed = next_light != state.effective_light_theme;
        if !force && !theme_changed {
            return Ok(());
        }

        let new_visuals = build_visuals(
            &state.source_frames,
            &state.source_sleep,
            next_light,
            state.settings,
        )?;
        let old_visuals = std::mem::replace(&mut state.visuals, new_visuals);
        state.effective_light_theme = next_light;
        state.last_tray_icon = 0;
        update_tray_icon_if_changed(state, true);
        update_tray_tooltip_if_changed(state, true);
        sync_overlay(state);
        destroy_visuals(&old_visuals);
        Ok(())
    }
