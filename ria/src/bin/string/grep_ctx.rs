// fn main() {
//     let haystack = "\
//     Every face, every shop,
//     bedroom window, public-house, and
//     dark square is a picture
//     feverishly turned--in search of what?
//     It is the same with books.
//     What do we seek
//     through millions of pages?";
//     let needle = "pages";
//     let ctx_length = 1;
//     let mut tags: Vec<usize> = vec![];
//     let mut ctx: Vec<Vec<(usize, String)>> = vec![];
//     for (i, line) in haystack.lines().enumerate() {
//         if line.contains(needle) {
//             tags.push(i);
//             let tag_v = vec![];
//             ctx.push(tag_v);
//         }
//     }
//     println!("{:?}", tags);
//     for (i, line) in haystack.lines().enumerate() {
//         for (j, tag) in tags.iter().enumerate() {
//             let lower_bound = tag.saturating_sub(ctx_length);
//             let upper_bound = tag + ctx_length;
//             if i >= lower_bound && i <= upper_bound {
//                 let v = (i, line.to_string());
//                 println!("{:?}", v);
//                 ctx[j].push(v);
//             }
//         }
//     }
//     println!("{:?}", ctx);
//     for local_ctx in ctx.iter() {
//         for (i, ctx) in local_ctx {
//             println!("{} {}", i, ctx);
//         }
//     }
// }
// fn main() {
//     let haystack = "\
//     Every face, every shop,
//     bedroom window, public-house, and
//     dark square is a picture
//     feverishly turned--in search of what?
//     It is the same with books.
//     What do we seek
//     through millions of pages?";
//     let needle = "oo";
//     let ctx_length = 1;

//     let mut tags_and_context: Vec<Vec<(usize, String)>> = vec![];
//     let mut buffer: Vec<(usize, String)> = vec![];

//     for (i, line) in haystack.lines().enumerate() {
//         // Add the current line to the buffer
//         buffer.push((i, line.to_string()));

//         // Check if the buffer exceeds the maximum size
//         if buffer.len() > ctx_length * 2 {
//             buffer.remove(0);
//         }

//         // If the current line contains the needle
//         if line.contains(needle) {
//             // Capture the current context
//             let mut context = buffer.clone();

//             // Fetch additional lines for the upper context
//             for (j, extra_line) in haystack.lines().enumerate().skip(i + 1).take(ctx_length) {
//                 context.push((j, extra_line.to_string()));
//             }

//             tags_and_context.push(context);
//         }
//     }

//     // Print the tags and contexts
//     for local_ctx in &tags_and_context {
//         for (i, ctx) in local_ctx {
//             println!("{} {}", i, ctx);
//         }
//         println!("----");
//     }
// }

fn main() {
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";
    let needle = "?";
    let ctx_length = 1;

    let mut buffer: Vec<(usize, String)> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        // adding line into a rolling bufer
        buffer.push((i, line.to_string()));
        // check buffer size
        if buffer.len() > ctx_length * 2 {
            buffer.remove(0);
        }
        // check the needle
        if line.contains(needle) {
            // println!("{:?}", buffer);
            let mut context = buffer.clone();
            for (j, line_after) in haystack.lines().enumerate().skip(i + 1).take(ctx_length) {
                context.push((j, line_after.to_string()));
            }
            ctx.push(context.clone());
        }
    }
    // println!("{:?}", ctx);
    for line in &ctx {
        for &(idx, ref content) in line {
            println!("{} {}", idx, content);
        }
    }
}
