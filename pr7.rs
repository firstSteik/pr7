use std::iter;

fn main() {
    let num_triangles = 5; // Change this to adjust the number of triangles
    draw_christmas_tree(num_triangles);
}

fn draw_christmas_tree(num_triangles: usize) {
    let max_width = 4 * num_triangles - 1;
    
    // Draw triangles
    (1..=num_triangles).for_each(|triangle| {
        let triangle_height = triangle * 2;
        (1..=triangle_height).for_each(|row| {
            let stars = 2 * row - 1;
            let padding = (max_width - stars) / 2;
            println!("{}{}{}", 
                " ".repeat(padding), 
                "*".repeat(stars), 
                " ".repeat(padding)
            );
        });
    });
    
    // Draw trunk
    let trunk_width = 3;
    let trunk_height = 2;
    let trunk_padding = (max_width - trunk_width) / 2;
    iter::repeat(()).take(trunk_height).for_each(|_| {
        println!("{}{}{}", 
            " ".repeat(trunk_padding), 
            "|".repeat(trunk_width), 
            " ".repeat(trunk_padding)
        );
    });
}