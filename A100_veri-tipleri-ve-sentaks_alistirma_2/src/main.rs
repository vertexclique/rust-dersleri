// 🌋: Alistirma 2: Arrayler ve for donguleri
//
// Ornek 1: For donguleri asagidaki gibidir.

// fn main() {
//     let array = [10, 20, 30, 40];
//     print!("Array'i iterate ediyoruz:");
//     for n in array {
//         print!(" {n}");
//     }
//     println!();
//
//     print!("Bir exclusive range'i iterate ediyoruz:");
//     for i in 0..3 {
//         print!(" {}", array[i]);
//     }
//     println!();
//
//     print!("Bir inclusive range'i iterate ediyoruz:");
//     for i in 0..=3 {
//         print!(" {}", array[i]);
//     }
//     println!();
// }

// 1. Matrisin transpozunu alan fonksiyonu yaziniz.
// Bir matrisin sol ust kosesinden sag alt kosesine cizilen bir cizgide
// matrisin elemanlarinin yer degistirilmesine matris transpoze denir.
//
// [ 1, 2, 3]        [ 1, 4, 7]
// [ 4, 5, 6] ---->  [ 2, 5, 8]
// [ 7, 8, 9]        [ 3, 6, 9]
//
// 2. Pretty print fonksiyonunu yazin.
// Soyle bir matris verildiginde
//
// let matrix = [
//     [101, 102, 103],
//     [201, 202, 203],
//     [301, 302, 303],
// ];
//
// bunu bastirmasini istiyoruz:
// ===========
// 101 102 103
// 201 202 203
// 301 302 303
// ===========
// 3. Transpoze fonksiyonunu arrayler yerine slicelar kullanarak yazabilir miydik?
// Odev... Yazin...
// fn transpoze(matrix: &[&[i32]]) -> &[&[i32]] {
// 4. Transpoze fonksiyonunuz kare olmayan matris icin de calisiyor mu?
// 5. Bu transpoze fonksiyonunu unutmayin.
// 6. Daha az memory kullanarak bu kodu yazabilir miydiniz? Dusunun.

#![allow(unused_variables, dead_code)]

fn transpoze(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!("Transpose fonksiyonunu yazin.")
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    todo!("Bir matrisi alip guzelce matris seklinde bastiran fonksiyonu yazin.")
}

fn main() {
    let matris = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matris:");
    pretty_print(&matris);

    let transpoze = transpoze(matris);
    println!("transpoze edilmis matris:");
    pretty_print(&transpoze);
}