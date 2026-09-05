    fn create_icon(
        frame: &FramePixels,
        crop: AlphaBounds,
        light_theme: bool,
        size: u32,
    ) -> Result<Hicon, &'static str> {
        let target = size.clamp(12, ICON_CANVAS);
        let crop_width = crop.width.max(1);
        let crop_height = crop.height.max(1);

        // Normalize the opaque artwork, not the source PNG canvas. This keeps the
        // sleeping pose and running animation at the same apparent tray scale while
        // preserving each sprite's aspect ratio and transparent padding.
        let (draw_width, draw_height) = if crop_width >= crop_height {
            let height = ((target as u64 * crop_height as u64 + crop_width as u64 / 2)
                / crop_width as u64) as u32;
            (target, height.max(1))
        } else {
            let width = ((target as u64 * crop_width as u64 + crop_height as u64 / 2)
                / crop_height as u64) as u32;
            (width.max(1), target)
        };

        let offset_x = (ICON_CANVAS - draw_width) / 2;
        let offset_y = (ICON_CANVAS - draw_height) / 2;
        let cat_channel = if light_theme { 0u8 } else { 255u8 };
        let mut canvas = vec![0u8; (ICON_CANVAS * ICON_CANVAS * 4) as usize];

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
                let destination_index = ((destination_y * ICON_CANVAS + destination_x) * 4) as usize;
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
                    width: ICON_CANVAS as i32,
                    height: -(ICON_CANVAS as i32),
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
            let color_bitmap = CreateDIBSection(0, &info, DIB_RGB_COLORS, &mut dib_bits, 0, 0);
            if color_bitmap == 0 || dib_bits.is_null() {
                return Err("CreateDIBSection failed");
            }

            copy_nonoverlapping(canvas.as_ptr(), dib_bits as *mut u8, canvas.len());

            let mask_bitmap = CreateBitmap(
                ICON_CANVAS as i32,
                ICON_CANVAS as i32,
                1,
                1,
                null(),
            );
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

    fn build_icon_set(
        frames: &[FramePixels],
        light_theme: bool,
        size: u32,
    ) -> Result<[Hicon; FRAME_COUNT], &'static str> {
        if frames.len() != FRAME_COUNT {
            return Err("expected exactly five running cat frames");
        }
        let width = frames[0].width;
        let height = frames[0].height;
        if frames.iter().any(|frame| frame.width != width || frame.height != height) {
            return Err("running cat frame dimensions do not match");
        }

        let crop = union_alpha_bounds(frames);
        let mut icons = [0; FRAME_COUNT];
        for index in 0..FRAME_COUNT {
            match create_icon(&frames[index], crop, light_theme, size) {
                Ok(icon) => icons[index] = icon,
                Err(error) => {
                    destroy_icon_set(&icons);
                    return Err(error);
                }
            }
        }
        Ok(icons)
    }

    fn destroy_icon_set(icons: &[Hicon; FRAME_COUNT]) {
        unsafe {
            for icon in icons.iter().copied().filter(|icon| *icon != 0) {
                DestroyIcon(icon);
            }
        }
    }

    fn notify_data(hwnd: Hwnd, icon: Hicon) -> NotifyIconDataW {
        let mut data: NotifyIconDataW = unsafe { zeroed() };
        data.cb_size = size_of::<NotifyIconDataW>() as Dword;
        data.hwnd = hwnd;
        data.id = 1;
        data.flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        data.callback_message = TRAY_CALLBACK;
        data.icon = icon;

        let label = wide("CatCPU");
        let copy_len = (label.len() - 1).min(data.tip.len() - 1);
        data.tip[..copy_len].copy_from_slice(&label[..copy_len]);
        data
    }

    fn add_tray_icon(hwnd: Hwnd, icon: Hicon) -> bool {
        let mut data = notify_data(hwnd, icon);
        unsafe { Shell_NotifyIconW(NIM_ADD, &mut data) != 0 }
    }

    fn update_tray_icon(hwnd: Hwnd, icon: Hicon) {
        let mut data = notify_data(hwnd, icon);
        data.flags = NIF_ICON;
        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    fn remove_tray_icon(hwnd: Hwnd) {
        let mut data = notify_data(hwnd, 0);
        data.flags = 0;
        unsafe {
            Shell_NotifyIconW(NIM_DELETE, &mut data);
        }
    }
