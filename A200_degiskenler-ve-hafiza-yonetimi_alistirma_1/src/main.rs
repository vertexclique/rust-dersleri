// TODO: Implementasyonlari bitirince bunu silin
#![allow(unused_variables, dead_code)]

struct Kutuphane {
    kitaplar: Vec<Kitap>,
}

struct Kitap {
    baslik: String,
    yil: u16,
}

impl Kitap {
    // Bu gordugunuz metod `yapici` dedigimiz metodumuz
    fn new(baslik: &str, yil: u16) -> Kitap {
        Kitap {
            baslik: String::from(baslik),
            yil,
        }
    }
}

// Asagida yazdigimiz kitaplari formatlayarak bastirmamiza yariyor {}.
impl std::fmt::Display for Kitap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.baslik, self.yil)
    }
}

impl Kutuphane {
    fn new() -> Kutuphane {
        unimplemented!()
    }

    //fn len(self) -> usize {
    //    unimplemented!()
    //}

    //fn is_empty(self) -> bool {
    //    unimplemented!()
    //}

    //fn kitap_ekle(self, Kitap: Kitap) {
    //    unimplemented!()
    //}

    //fn kitaplari_bastir(self) {
    //    unimplemented!()
    //}

    //fn en_eski_kitap(self) -> Option<&Kitap> {
    //    unimplemented!()
    //}
}

// Bu istenen davranışı gösterir. Aşağıdaki kodun yorumunu kaldırın ve
// eksik yöntemleri uygulayın. "self" parametresi de dahil olmak üzere tum 
// metod imzalarıni guncellemeniz gerekecek! Yapabilirsin
// `main` içindeki değişken atamalarini da güncellemeniz gerekebilir.
fn main() {
    let kutuphane = Kutuphane::new();

    //println!("Kutuphane bos: {}", kutuphane.is_empty());
    //
    //kutuphane.kitap_ekle(Kitap::new("Lord of the Rings", 1954));
    //kutuphane.kitap_ekle(Kitap::new("Alice's Adventures in Wonderland", 1865));
    //
    //kutuphane.kitaplari_bastir();
    //
    //match kutuphane.en_eski_kitap() {
    //    Some(kitap) => println!("En eski kitabim {kitap}"),
    //    None => println!("Kutuphanem bos!"),
    //}
    //
    //println!("Kutuphanemiz {} kitap iceriyor", kutuphane.len());
}