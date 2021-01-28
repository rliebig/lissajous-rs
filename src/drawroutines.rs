fn draw_circle(buffer: &mut Vec<u32>, x : i32, y : i32, radius : f32) {
    //todo: boundary check.
    for i in 0..360 {
        let x1 = radius * ((i as f32 * std::f32::consts::PI) / 180.0).cos();
        let y1 = radius * ((i as f32 * std::f32::consts::PI) / 180.0).sin();
        let conv_x1 : i32 = x1 as i32;
        let conv_y1 : i32 = y1 as i32;

        let coord_x = (x + conv_x1) as u32;
        let coord_y = (y + conv_y1) as u32;
        buffer[(coord_x + WIDTH as u32 * coord_y) as usize] = std::u32::MIN;
    }
}

fn circle_coordinate(degree : i32, x : i32, y : i32, radius : f32) -> (i32, i32) {
    if degree < 0 ||degree > 360 {
        return (0,0);
    }

    let x1 = radius * ((degree as f32 * std::f32::consts::PI) / 180.0).cos();
    let y1 = radius * ((degree as f32 * std::f32::consts::PI) / 180.0).sin();
    let conv_x1 : i32 = x1 as i32;
    let conv_y1 : i32 = y1 as i32;

    let coord_x = (x + conv_x1) as i32;
    let coord_y = (y + conv_y1) as i32;

    return (coord_x, coord_y);
}

fn draw_filled_circle(buffer: &mut Vec<u32>, x :i32, y: i32, radius: f32) {
    for i in (-radius as i32)..(radius as i32) {
        let height = ((radius * radius) - (i as f32 * i as f32)).sqrt() as i32;
        for j in (-height as i32)..(height as i32) {
            let coord_x = (x + i) as u32;
            let coord_y = (y + j) as u32;
            buffer[(coord_x + WIDTH as u32 * coord_y) as usize] = std::u32::MIN;
        }
    }
}

fn draw_filled_circle_color(buffer: &mut Vec<u32>, x :i32, y: i32, radius: f32, color: u32) {
    for i in (-radius as i32)..(radius as i32) {
        let height = ((radius * radius) - (i as f32 * i as f32)).sqrt() as i32;
        for j in (-height as i32)..(height as i32) {
            let coord_x = (x + i) as u32;
            let coord_y = (y + j) as u32;
            buffer[(coord_x + WIDTH as u32 * coord_y) as usize] = color;
        }
    }
}

fn base_circle_update(buffer : &mut Vec<u32>, x :i32, y: i32, radius: f32, step : i32) -> (i32, i32){
    draw_circle(buffer, x, y, radius);
    let my_pair = circle_coordinate(step, x, y, radius);
    draw_filled_circle(buffer, my_pair.0, my_pair.1, 5.0);
    return my_pair
}

fn euclidean_distance(first_point : (i32, i32), second_point : (i32, i32)) -> i32 {
    let x = first_point.0 + second_point.0;
    let y = first_point.0 + second_point.0;
    return (((x + y) * (x + y)) as f32).sqrt() as i32;
}

fn handle_follow_line(my_pair : (i32,i32),
                      third_pair : (i32, i32),
                      follow_line: &mut Vec<(i32, i32)>,
                      second_line: &mut Vec<(i32,i32)>) {
    if !follow_line.is_empty() {
        let mut last_point = follow_line.last().unwrap();
        //harcoded offsets...
        let mut current_point = (third_pair.0, my_pair.1);
        if (euclidean_distance(last_point.clone(), current_point)) != 1 {
            //naive line drawing - did not work out so well
            if last_point.0 > current_point.0 {
                //swap to make the algorithm work -- needs improvement for readability
                let dx = last_point.0 - current_point.0;
                let dy = last_point.1 - current_point.1;

                for x_way in current_point.0 .. last_point.0 {
                    let y_way = current_point.1 + dy * (x_way - current_point.0) / dx;
                    second_line.push((x_way, y_way));
                }
            }
            let dx = current_point.0 - last_point.0;
            let dy = current_point.1 - last_point.1;

            for x_way in last_point.0 .. current_point.0 {
                let y_way = last_point.1 + dy * (x_way - last_point.0) / dx;
                second_line.push((x_way, y_way));
            }
            //Bresenham algorithm for line drawing - did also not work out to well. Oh my.
            //let dx : f32 = current_point.0 as f32 - last_point.0 as f32;
            //let dy : f32 = current_point.1 as f32 - last_point.1 as f32;
            //let mut derr : f32 = (dx / dy).abs() as f32;
            //let mut err : f32 = 0.0;
            //let mut way_y : f32 = last_point.1 as f32;
            //for x_way in last_point.0 .. current_point.0 {
            //    second_line.push((x_way, way_y as i32));
            //    err = err + derr;
            //    if err >= 0.5 {
            //        way_y = way_y + dy.signum();
            //        err = err - 1.0;
            //    }
            //}
        }
    }
}
