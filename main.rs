/*
Program: select_sort
Author : José Luis Márquez Alvarado
Date   : 14/03/2023
Version: 1.0
*/

use std::io::{
    stdin ,
    stdout,
    Write
};

fn main(){
    let mut colection: Vec<f32> = Vec::new();
    let quantity: u32 = u32_input( "Insert your amount number: " );
    
    for item in 1..quantity+1 {
        let comment: String = (item).to_string()+": ";
        colection.push( f32_input( &comment ) );
    }

    sort_selection( &mut colection );
    println!("{:?}", colection);
}

fn sort_selection( colection: &mut Vec<f32> ){
    let mut usize1  : usize= 0;
    let mut usize2  : usize   ;
    let mut auxiliar: f32     ;

    while usize1 < colection.len() {

        usize2 = usize1;
        while usize2 < colection.len() {
            if colection[ usize1 ] > colection[ usize2 ] {
                auxiliar            = colection[ usize1 ];
                colection[ usize1 ] = colection[ usize2 ];
                colection[ usize2 ] = auxiliar;
            }
            usize2 += 1;
        }
        usize1 += 1;
    }
}

fn u32_input( comment: &str ) -> u32 {
    let number: u32 = loop {
        let value: u32 = match input( &comment ).parse() {
            Ok( u32_number ) => u32_number,
            Err(     _     ) => {
                println!( "Please insert a integer number..." );
                continue;
            }
        };

        break value;
    };

    number
}

fn f32_input( comment :&str ) -> f32 {
    let number: f32 = loop {
        let value: f32 = match input( &comment ).parse() {
            Ok( f32_number ) => f32_number,
            Err(     _     ) => {
                println!("Please insert a floatingg number...");
                continue;
            }
        };

        break value;
    };

    number
}

fn input( comment: &str ) -> String {
    print!( "{}", comment );
    stdout().flush().unwrap();
    let mut value: String = String::new();
    stdin().read_line( &mut value ).expect( "Failed to read line" );

    (value.trim()).to_string()
}