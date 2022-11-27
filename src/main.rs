use std::io;

fn main() {
    ask_matrice();
}

fn ask_matrice(){
    let width: usize = ask_number("width : ");
    let mut matrice: Vec<Vec<usize>> = Vec::new();
    let mut solution: Vec<usize> = Vec::new();
    for i in 0..width{
        matrice.push(Vec::new());
        for _j in 0..width{
            matrice[i].push(ask_number("value : "));
            display_matrice(&matrice, &solution);
        }
        solution.push(ask_number("solution : "));
        display_matrice(&matrice, &solution);
    }
}

fn ask_number(msg: &str) -> usize{
    let number: usize;
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    number = input.trim().parse().expect("Please type a number!");
    return number;
}

fn display_matrice(matrice: &Vec<Vec<usize>>, solution: &Vec<usize>){
    println!("");
    for i in 0..matrice.len(){
        for j in 0..matrice[i].len(){
            print!("{} ", matrice[i][j]);
        }
        if solution.len() > i {
            print!("| {}", solution[i]);
        }
        println!("");
    }
    println!("");
}

fn gauss_pivot(matrice: &Vec<Vec<usize>>, solution: &Vec<usize>) -> Vec<Vec<usize>>{
    let mut new_matrice = matrice.clone();

    //TODO, use divide_row, swap_row, substract_row

    return new_matrice;
}

fn divide_row(row: &Vec<usize>, divisor: usize) -> Vec<usize>{
    let mut new_row: Vec<usize> = Vec::new();
    for i in 0..row.len(){
        new_row.push(row[i] / divisor);
    }
    return new_row;
}

fn swap_row(matrice: &Vec<Vec<usize>>, row1: usize, row2: usize) -> Vec<Vec<usize>>{
    let mut new_matrice = matrice.clone();
    let tmp = new_matrice[row1].clone();
    new_matrice[row1] = new_matrice[row2].clone();
    new_matrice[row2] = tmp;
    return new_matrice;
}

fn substract_row(row1: &Vec<usize>, row2: &Vec<usize>) -> Vec<usize>{
    let mut new_row: Vec<usize> = Vec::new();
    for i in 0..row1.len(){
        new_row.push(row1[i] - row2[i]);
    }
    return new_row;
}

fn multiply_matrice(matrice: &Vec<Vec<usize>>, multiplier: usize) -> Vec<Vec<usize>>{
    let mut new_matrice = matrice.clone();
    for i in 0..new_matrice.len(){
        for j in 0..new_matrice[i].len(){
            new_matrice[i][j] *= multiplier;
        }
    }
    return new_matrice;
}

fn multiply_matrice_matrice(matrice1: &Vec<Vec<usize>>, matrice2: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let mut new_matrice: Vec<Vec<usize>> = Vec::new();
    if matrice1[0].len() != matrice2.len() {
        panic!("Matrice1 width must be equal to matrice2 height");
    } 
    for i in 0..matrice1.len(){
        new_matrice.push(Vec::new());
        for j in 0..matrice2[0].len(){
            new_matrice[i].push(0);
            for k in 0..matrice1[0].len(){
                new_matrice[i][j] += matrice1[i][k] * matrice2[k][j];
            }
        }
    }
    return new_matrice;
}