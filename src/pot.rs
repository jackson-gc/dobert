



fn draw_pot(trk: &mut Stdout) -> Result<()> {
    let window_width: u16 = size()?.0;
    let window_length: u16 = size()?.1;

    if window_width < MIN_WINDOW_WIDTH {
        panic!("Window width is too small (must be >{})", MIN_WINDOW_WIDTH);
    }

    if window_length < MIN_WINDOW_LENGTH {
        panic!("Winow length is too small (must be >{})", MIN_WINDOW_LENGTH);
    }

    let start_index: u16 = window_width - MIN_WINDOW_WIDTH;
    let margin: u16 = start_index / 4;
    let size: u16 = window_width - margin * 2;
    let pot_depth: u16 = window_length / 8;
    let depth: u16 = window_length - pot_depth;

    let clr: Color = Color::Rgb{r: 160, g: 130, b: 90};
    let pot_hard_mat: StyledContent<char> = '█'.with(clr);

    paint_rect(trk, pot_hard_mat, Rect {
        s_pnt: Point {
            x: margin,
            y: depth
        },
        e_pnt: Point {
            x: margin + size,
            y: depth + 1
        }
    })?;

    let mut draw_pnt = Point {
        x: margin + LIP_SIZE,
        y: depth
    };

    let mut draw_shift = Shifter {
        gap: size - ((margin / 4) + LIP_SIZE * 2), 
        x_shift: 1, 
        y_shift: 1
    };
    
    for i in 1..pot_depth {
        if i & 2 == 0 {
            draw_shift.x_shift = 0;
        } else {
            draw_shift.x_shift = 1;
        }

        if i == pot_depth - pot_depth / 4 {
            let end_width = draw_pnt.x + draw_shift.gap;

            paint_rect(trk, pot_hard_mat, Rect {
                s_pnt: draw_pnt.clone(),
                e_pnt: Point {
                    x: end_width,
                    y: draw_pnt.y + 1
                }
            })?;
        }
        paint_outline(trk, pot_hard_mat, &mut draw_pnt, &mut draw_shift)?;
    }

    let end_width: u16 = draw_pnt.x + draw_shift.gap + 1;
    let end_length: u16 = draw_pnt.y;
    paint_rect(trk, pot_hard_mat, Rect {
        s_pnt: draw_pnt,
        e_pnt: Point {
            x: end_width,
            y: end_length
        }
    })?;

    Ok(())
}
