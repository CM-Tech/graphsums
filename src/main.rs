extern crate piston_window;

use piston_window::*;

fn sorted(x: (usize, usize)) -> (usize, usize) {
    if x.0 > x.1 {
        (x.1, x.0)
    } else {
        x
    }
}

fn calc(edges: &mut Vec<(usize, usize)>, points: &Vec<(f64, f64)>) -> [i32; 3] {
    edges.sort();
    edges.dedup_by(|x, y| x.0 == x.1 || x == y);
    let mut total = [0; 3];
    let mut c = vec![0; points.len()];
    loop {
        let mut sum = 0;
        for i in edges.iter() {
            sum += c[i.0] * c[i.1];
        }
        total[sum % 3] += 1;
        /*for i in edges.iter() {
            total[(c[i.0] * c[i.1]) % 3] += 1;
        }*/

        if c == vec![2; points.len()] {
            break;
        }

        let mut add_next = 1;
        for n in c.iter_mut().rev() {
            *n += add_next;
            add_next = 0;
            if *n == 3 {
                *n = 0;
                add_next = 1;
            }
        }
    }
    let min = total.iter().min().unwrap().clone();
    for x in total.iter_mut() {
        *x -= min
    }
    total
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Graph Sums!", [500; 2])
        .build()
        .unwrap();

    let font = include_bytes!("../assets/FiraCode.ttf");
    let mut glyphs =
        Glyphs::from_bytes(font, window.factory.clone(), TextureSettings::new()).unwrap();

    let mut points: Vec<(f64, f64)> = vec![];
    let mut edges: Vec<(usize, usize)> = vec![];
    let mut cursor = (0.0, 0.0);
    let mut click: Option<usize> = None;
    let mut total = [0; 3];
    let mut mode = 0;
    while let Some(e) = window.next() {
        e.mouse_cursor(|x, y| {
            if mode == 1 {
                if let Some(x) = click {
                    points[x] = cursor;
                }
            }
            cursor = (x, y)
        });
        match e.press_args() {
            Some(Button::Mouse(_)) => {
                if mode == 0 {
                    mode = 1
                }
                if let Some(p) = points.iter().position(|p| {
                    (cursor.0 < p.0 + 10.0) && (cursor.0 > p.0 - 10.0) && (cursor.1 < p.1 + 10.0)
                        && (cursor.1 > p.1 - 10.0)
                }) {
                    if let Some(x) = click {
                        if p != x {
                            edges.push(sorted((x, p)));
                            total = calc(&mut edges, &points);
                            click = Some(p);
                        }
                    } else {
                        if mode > 1 {
                            points.remove(p);
                            edges.retain(|z| z.0 != p && z.1 != p);
                            for z in edges.iter_mut() {
                                if z.0 >= p {
                                    z.0 = z.0 - 1;
                                }
                                if z.1 >= p {
                                    z.1 = z.1 - 1;
                                }
                            }
                            total = calc(&mut edges, &points);
                        } else {
                            click = Some(p);
                        }
                    }
                } else {
                    if mode < 2 {
                        points.push(cursor);
                        if click.is_none() {
                            click = Some(points.len() - 1);
                        }
                    }
                    if let Some(x) = click {
                        edges.push(sorted((x, points.len() - 1)));
                        click = Some(points.len() - 1);
                    }
                    total = calc(&mut edges, &points);
                }
            }
            Some(Button::Keyboard(Key::LShift)) => {
                mode = 2;
                click = None;
            }
            Some(Button::Keyboard(Key::M)) => mode = 1,
            Some(Button::Keyboard(Key::K)) => {
                edges.clear();
                for i in 0..points.len() {
                    for j in i + 1..points.len() {
                        edges.push(sorted((i, j)));
                    }
                }
                total = calc(&mut edges, &points);
            }
            Some(Button::Keyboard(Key::C)) => {
                edges.clear();
                points.clear();
                click = None;
                total = calc(&mut edges, &points);
            }
            _ => (),
        }
        match e.release_args() {
            Some(Button::Keyboard(Key::LShift)) | Some(Button::Keyboard(Key::M)) => {
                mode = 0;
                click = None;
            }
            Some(Button::Mouse(_)) => mode = 0,
            _ => (),
        }
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            if let Some(x) = click {
                if let Some(p) = points.iter().position(|p| {
                    (cursor.0 < p.0 + 10.0) && (cursor.0 > p.0 - 10.0) && (cursor.1 < p.1 + 10.0)
                        && (cursor.1 > p.1 - 10.0)
                }) {
                    line(
                        [0.5, 0.5, 0.5, 1.0],
                        1.0,
                        [points[x].0, points[x].1, points[p].0, points[p].1],
                        c.transform,
                        g,
                    );
                } else {
                    line(
                        [0.5, 0.5, 0.5, 1.0],
                        1.0,
                        [points[x].0, points[x].1, cursor.0, cursor.1],
                        c.transform,
                        g,
                    );
                }
            }

            for &(x, y) in edges.iter() {
                line(
                    [1.0; 4],
                    1.0,
                    [points[x].0, points[x].1, points[y].0, points[y].1],
                    c.transform,
                    g,
                );
            }
            for (i, x) in (0..).zip(points.iter()) {
                ellipse(
                    if Some(i) == click {
                        [0.0, 0.0, 1.0, 1.0]
                    } else {
                        [1.0, 0.0, 0.0, 1.0]
                    },
                    [x.0 - 10.0, x.1 - 10.0, 20.0, 20.0],
                    c.transform,
                    g,
                );
            }
            text(
                [1.0; 4],
                20,
                &format!(
                    "|{} + {}ω + {}ω̄| = √{}",
                    total[0],
                    total[1],
                    total[2],
                    ((total[0] as f64 - (0.5) * (total[1] as f64) - (0.5) * (total[2] as f64))
                        .powf(2.0)
                        + (((3f64).sqrt() / 2.0) * (total[1] as f64)
                            - ((3f64).sqrt() / 2.0) * (total[2] as f64))
                            .powf(2.0))
                        .round()
                ),
                &mut glyphs,
                c.transform.trans(10.0, 25.0),
                g,
            ).unwrap();
            text(
                [1.0; 4],
                20,
                &format!("Edges: {}", edges.len()),
                &mut glyphs,
                c.transform.trans(10.0, 50.0),
                g,
            ).unwrap();
            text(
                [1.0; 4],
                20,
                &format!("Vertices: {}", points.len()),
                &mut glyphs,
                c.transform.trans(10.0, 75.0),
                g,
            ).unwrap();
        });
    }
}
