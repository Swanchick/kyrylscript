//
// Example of drawing a triangle in console
//


// ******
// *****
// ****
// ***
// **
// *


function draw_triangle(size int) {
    let column = 0;

    while column < size {
        column = column + 1;

        let row = 0;

        while row < size - column {
            row = row + 1;

            print("*");
        }

        println("");
    }
}

draw_triangle(7);
