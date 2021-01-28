
fn grid() {
    let mut buffer: Vec<u32> = vec![std::u32::MAX; WIDTH * HEIGHT];
    let mut window = Window::new(
        "lissajous-rs",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap();


    let circle_radius : i32 = 50;
    let circle_radius_float : f32 = circle_radius as f32; // for pure convenience.
    let circle_diameter = circle_radius * 2;
    let first_down_x: i32 = circle_diameter * 1;
    let first_down_y: i32 = circle_diameter * 2;

    let first_right_x: i32 = circle_diameter * 2;
    let first_right_y: i32 = circle_diameter * 1;

    let second_right_x: i32 = circle_diameter * 3;
    let second_right_y: i32 = circle_diameter * 1;

    let third_right_x: i32 = circle_diameter * 1;
    let third_right_y: i32 = circle_diameter * 3;

    let fifth_x : i32 = circle_diameter * 4;
    let fifth_y : i32 = circle_diameter * 1;

    let third_down_x: i32 = circle_diameter * 1;
    let third_down_y: i32 = circle_diameter * 4;

    let seventh_x : i32 = circle_diameter * 5;
    let seventh_y : i32 = circle_diameter * 1;

    let eight_x : i32 = circle_diameter * 1;
    let eight_y : i32 = circle_diameter * 5;

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut step : i32 = 0;
    let mut second_step: i32 = 0;
    let mut third_step : i32 = 0;
    let mut fourth_step : i32 = 0;

    let mut follow_line : Vec<(i32,i32)> = vec![];

    let mut step_interval : i32 = 1 * 2;
    let mut second_step_interval : i32 = 2 * 2;
    let mut third_step_interval : i32 = 3 * 2;
    let mut fourth_step_interval : i32 = 5 * 2;

    while window.is_open() {
        if step > 360 {
            follow_line.clear();
            return;
            step -= 360;
        }

        if second_step > 360 {
            second_step -= 360;
        }

        if third_step > 360 {
            third_step -= 360;
        }

        if fourth_step > 360 {
            fourth_step -= 360;
        }
        for val in buffer.iter_mut() {
            *val = std::u32::MAX;
        }
        for point in follow_line.iter() {
            draw_filled_circle(&mut buffer, point.0, point.1, 1.0);
        }

        draw_circle(&mut buffer, first_right_x, first_right_y, circle_radius_float);

        let my_pair : (i32, i32) = base_circle_update(&mut buffer, first_down_x, first_down_y, circle_radius_float, step);
        let second_pair : (i32, i32) = base_circle_update(&mut buffer, first_right_x, first_right_y, circle_radius_float, step);
        let third_pair : (i32, i32) = base_circle_update(&mut buffer, second_right_x, second_right_y, circle_radius_float, second_step);
        let fourth_pair : (i32, i32)= base_circle_update(&mut buffer, third_right_x, third_right_y, circle_radius_float, second_step);
        let fifth_pair : (i32, i32)= base_circle_update(&mut buffer, fifth_x, fifth_y,circle_radius_float, third_step);
        let sixth_pair : (i32, i32) = base_circle_update(&mut buffer, third_down_x, third_down_y, circle_radius_float, third_step);
        let four_one : (i32, i32) = base_circle_update(&mut buffer, seventh_x, seventh_y, circle_radius_float, fourth_step);
        let four_two : (i32, i32) = base_circle_update(&mut buffer, eight_x, eight_y, circle_radius_float, fourth_step);

        draw_filled_circle(&mut buffer, four_one.0, four_two.1, 5.0);
        follow_line.push((four_one.0, four_two.1));
        draw_filled_circle(&mut buffer, fifth_pair.0, four_two.1, 5.0);
        follow_line.push((fifth_pair.0, four_two.1));
        draw_filled_circle(&mut buffer, third_pair.0, four_two.1, 5.0);
        follow_line.push((third_pair.0, four_two.1));
        draw_filled_circle(&mut buffer, second_pair.0, four_two.1, 5.0);
        follow_line.push((second_pair.0, four_two.1));

        draw_filled_circle(&mut buffer, four_one.0, my_pair.1, 5.0);
        follow_line.push((four_one.0, my_pair.1));
        draw_filled_circle(&mut buffer, four_one.0, fourth_pair.1, 5.0);
        follow_line.push((four_one.0, fourth_pair.1));
        draw_filled_circle(&mut buffer, four_one.0, sixth_pair.1, 5.0);
        follow_line.push((four_one.0, sixth_pair.1));

        draw_filled_circle(&mut buffer, second_pair.0, sixth_pair.1, 5.0);
        follow_line.push((second_pair.0, sixth_pair.1));
        draw_filled_circle(&mut buffer, fifth_pair.0, sixth_pair.1, 5.0);
        follow_line.push((fifth_pair.0, sixth_pair.1));
        draw_filled_circle(&mut buffer, third_pair.0, sixth_pair.1, 5.0);
        follow_line.push((third_pair.0, sixth_pair.1));


        draw_filled_circle(&mut buffer, second_pair.0, fourth_pair.1, 5.0);
        follow_line.push((second_pair.0, fourth_pair.1));
        draw_filled_circle(&mut buffer, third_pair.0, fourth_pair.1, 5.0);
        follow_line.push((third_pair.0, fourth_pair.1));

        draw_filled_circle(&mut buffer, fifth_pair.0, fourth_pair.1, 5.0);
        follow_line.push((fifth_pair.0, fourth_pair.1));

        draw_filled_circle(&mut buffer, fifth_pair.0, my_pair.1, 5.0);
        follow_line.push((fifth_pair.0, my_pair.1));


        draw_filled_circle(&mut buffer,third_pair.0, my_pair.1, 5.0);
        follow_line.push((third_pair.0, my_pair.1));


        draw_filled_circle(&mut buffer, second_pair.0, my_pair.1, 5.0);
        follow_line.push((second_pair.0, my_pair.1));



        step += step_interval;
        second_step += second_step_interval;
        third_step += third_step_interval;
        fourth_step += fourth_step_interval;
        let mut owned_string = "".to_owned();
        owned_string.push_str(step.to_string().as_str());
        owned_string.push_str(".png");
        let path = Path::new(owned_string.as_str() );
        let mut file = match File::create(&path) {
            Err(why) => panic!("File open didn ot work. {}", why),
            Ok(file) => file,
        };

        png_encode_mini::write_rgba_from_u32(&mut file, &buffer, WIDTH as u32, HEIGHT as u32);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}