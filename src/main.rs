extern crate find_folder;
extern crate piston_window;

use piston_window::*;

fn calc(lines: &mut Vec<(usize, usize)>, points: &Vec<(f64, f64)>) -> [i32; 3] {
    lines.dedup_by(|x, y| x.0 == x.1 || x == y || (x.0 == y.1 && x.1 == y.0));
    let mut total = [0; 3];
    let mut c = vec![0; points.len()];
    loop {
        let mut sum = 0;
        for i in lines.iter() {
            sum += c[i.0] * c[i.1];
        }
        total[sum % 3] += 1;
        /*for i in lines.iter() {
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
    let font = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap()
        .join("FiraCode.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone(), TextureSettings::new()).unwrap();

    let mut points: Vec<(f64, f64)> = vec![];
    let mut lines: Vec<(usize, usize)> = vec![];
    let mut cursor = [0.0; 2];
    let mut click: Option<usize> = None;
    let mut total = [0; 3];
    let mut deleting = false;
    while let Some(e) = window.next() {
        e.mouse_cursor(|x, y| cursor = [x, y]);
        match e.press_args() {
            Some(Button::Mouse(_)) => {
                if let Some(p) = points.iter().position(|p| {
                    (cursor[0] < p.0 + 10.0) && (cursor[0] > p.0 - 10.0) && (cursor[1] < p.1 + 10.0)
                        && (cursor[1] > p.1 - 10.0)
                }) {
                    if let Some(x) = click {
                        lines.push((x, p));
                        click = None;
                    } else {
                        if deleting {
                            points.remove(p);
                            lines.retain(|z| z.0 != p && z.1 != p);
                            for z in lines.iter_mut() {
                                if z.0 > p {
                                    z.0 = z.0 - 1;
                                }
                                if z.1 > p {
                                    z.1 = z.1 - 1;
                                }
                            }
                        } else {
                            click = Some(p);
                        }
                    }
                } else {
                    if !deleting {
                        points.push((cursor[0], cursor[1]));
                    }
                    if let Some(x) = click {
                        lines.push((x, points.len() - 1));
                        click = None;
                    }
                }
                total = calc(&mut lines, &points);
            }
            Some(Button::Keyboard(Key::LShift)) => {
                deleting = true;
                click = None;
            }
            Some(Button::Keyboard(Key::C)) => {
                lines.clear();
                points.clear();
                click = None;
                total = calc(&mut lines, &points);
            }
            _ => (),
        }
        if let Some(Button::Keyboard(Key::LShift)) = e.release_args() {
            deleting = false;
        }
        let size = window.size().width as f64;
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            if let Some(x) = click {
                line(
                    [1.0; 4],
                    1.0,
                    [points[x].0, points[x].1, cursor[0], cursor[1]],
                    c.transform,
                    g,
                );
            }
            for &(x, y) in lines.iter() {
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
                &format!("{} + {}ω + {}ω̄", total[0], total[1], total[2]),
                &mut glyphs,
                c.transform.trans(size / 2.0 - 70.0, 70.0),
                g,
            ).unwrap();
            text(
                [1.0; 4],
                20,
                &format!("Lines: {}", lines.len()),
                &mut glyphs,
                c.transform.trans(size / 2.0 - 50.0, 95.0),
                g,
            ).unwrap();
            text(
                [1.0; 4],
                20,
                &format!("Vertices: {}", points.len()),
                &mut glyphs,
                c.transform.trans(size / 2.0 - 70.0, 120.0),
                g,
            ).unwrap();
        });
    }
}
