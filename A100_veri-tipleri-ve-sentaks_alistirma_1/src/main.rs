// 🌋: Alistirma 1: Tipler arasi otomatik donusumler
//
// Rust'ta tipler arasi otomatik donusum yoktur.
// Yazmadan once ne kullanacaginizi ve neye donustureceginizi
// bilmeniz veya onceden verinin sigacagi bir veri tipi secmeniz gereklidir.

fn carp(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // 1.Kodu derlenecek hale getirin.
    println!("{x} * {y} = {}", carp(x, y));

    // 2. Bu kod sizce derlenecek hale gelir mi?
    // 3. Bu kod derlense de calisir mi?
    let z: i32 = 12783891;
    println!("{z} * {y} = {}", carp(z, y));

    // 4. x ve y nin degerlerini f32, bool ve i128 gibi veri tipleriyle degistirip derleyin.
}