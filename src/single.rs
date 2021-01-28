include!("drawroutines.rs");

const WIDTH : usize = 600;
const HEIGHT : usize= 600;
fn single() {

    let mut buffer: Vec<u32> = vec![std::u32::MAX; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap();

    let x : i32 = 100;
    let y : i32 = 200;
    let second_x : i32 = 200;
    let second_y : i32 = 100;

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut step = 0;
    let mut second_step = 0;
    let mut follow_line : Vec<(i32,i32)> = vec![];
    let mut second_line : Vec<(i32,i32)> = vec![];


    let mut step_interval : i32 = 2;
    let mut second_step_interval : i32 = 2;


    while window.is_open() {
        if step > 360 {
            step -= 360;
        }

        if second_step > 360 {
            second_step -= 360;
        }

        for val in buffer.iter_mut() {
            *val = std::u32::MAX;
        }

        for point in follow_line.iter() {
            draw_filled_circle(&mut buffer, point.0, point.1, 2.0);
        }

        for point in second_line.iter() {
            println!("{}, {}", point.0, point.1);
            draw_filled_circle(&mut buffer, point.0, point.1, 2.0);
        }

        let my_pair : (i32, i32) = base_circle_update(&mut buffer, x,y,50.0,step);
        let second_pair : (i32, i32) = base_circle_update(&mut buffer, second_x, second_y, 50.0,second_step);

        draw_filled_circle(&mut buffer, second_pair.0,my_pair.1, 5.0);
        follow_line.push((second_pair.0, my_pair.1));
        //handle_follow_line(my_pair,second_pair,&mut follow_line, &mut second_line);

        step += step_interval;
        second_step += second_step_interval;
        window.set_title(step_interval.to_string().as_str());

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => {
                        step_interval += 2;
                        if step_interval == 0 {
                            step_interval = 1;
                        }
                    },
                    Key::S => step_interval -= 2,
                    Key::Up => {
                        second_step_interval += 2;
                        if second_step_interval == 0 {
                            second_step_interval = 1;
                        }
                    },
                    Key::Down => second_step_interval -= 2,
                    Key::C => {follow_line.clear();second_line.clear();},
                    _ => (),
                }
            }
        }) ;

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}