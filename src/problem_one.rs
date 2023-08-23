/*
the first problem requires you to make  a program that prints out the values of A , B and C. where A is the sum of the square of 2 parameters x and y
*/

pub fn p_1(x:i32 , y : i32){
    println!("A = x^2 + y^2 \nwhen x = {}, and y= {} \n A = {} ", x, y, x.pow(2) + y.pow(2));

    println!("\n\nB = A^2 + xy \nwhen x = {}, and y= {} \n B = {} ", x, y, (x.pow(2) + y.pow(2))+ x*y);

    println!("\n\nC = (A+B)^2 \nwhen x = {}, and y= {} \n C = {} ", x, y, ((x.pow(2) + y.pow(2)) + ( (x.pow(2) + y.pow(2))+ x*y)).pow(2))
} 