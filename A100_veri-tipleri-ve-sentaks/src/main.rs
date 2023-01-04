// ✨ Asama 1. Merhaba 🌍!

// Her rust programi entrypoint dedigimiz fonksiyondan olusur.
// `main` isimli fonksiyon programin baslama noktasidir.
//
// Fonksiyonlar `fn` ile sozuyle tanimlanir.
// Rust'ta stringler UTF-8 olarak tanimlanir.
// UTF-8 bir karakter kodlama cesididir.
// println! ve unlemle biten cogu sozdizimine `makro` denir.

fn main() {
    println!("Hello 🌍!");
}

// ✨ Asama 2. Sentaks'a bir bakis.

// fn main() {
//     // Bloklar ve scopelar suslu parantez ile birbirinden ayrilirlar.
//     // Yazdiginiz fonksiyonun disinda baska scope turleri de vardir.
//     // Ama o scopelara burada deginmeyecegiz. Elementary ve Advanced Rust
//     // egitiminde encapsulation ve baska scopelar uzerine konusacagiz.
//
//     let mut x: i32 = 6;  // Degisebilir degisken tanimi
//     print!("{x}");       // printf ile ayni gorevi goren makro `println!`
//     while x != 1 {       // Cogunlukla ifadelerin cevresinde normal parantez olmaz.
//         if x % 2 == 0 {  // Bildiginiz matematik diger diller ile ayni sekilde calisir.
//             x = x / 2;
//         } else {
//             x = 3 * x + 1;
//         }
//         print!(" -> {x}");
//     }
//     println!();
// }

// ✨ Asama 3. Ya bu dili diger dillerden farkli kilan nedir?

// Derleme zamaninda hafiza guvenligini saglar.
// UB (undefined behavior) nam-i diger Bilinmeyen Davranis (BD)'i onler.
// Modern dillere ait tum ozelliklere sahiptir.

// Soru: Ne yaparsam yapayim hicbir zaman kotu kod yazamaz miyim?
// Cevap: Unsafe Rust (Guvensiz Rust) yazmadigin surece %99 bir sorun olmaz.

// Soru: `unsafe` kelimesini kullanmadan herhangi bir hata olmadan yazabilirim o zaman?
// Cevap: %99 diyelim. %1 hata yapma olasiligin ise cok nadir bir durum. O da su sekilde:
// Simdilik cok onemi olmayan ama safe Rust ile de bazi yanlis seyler yapabileceginizi gosteren kod.

// fn main() {
//     let x = Box::new(1337_u64);
//     let ptr = Box::leak(x); // Artik program x'i takip etmeyecek. Destructor (yikici) cagirilmadi.
//     drop(ptr); // Pointeri da tam burada kaybettik :(
// }

// ✨ Asama 4. Veri tipleri: Scalars - Skaler veri tipi

// fn main() {
//
//     // Integer Kralligi
//
//     // i8 - bir byte uzunlugundaki isaretli integer - 3 farkli tanim. 3 ayni sey
//     let bir_i8: i8 = 1_i8;
//     let boyle_de_bir_i8: i8 = 1i8;
//     let bu_da_bir_i8: i8 = 1;
//     // i16 - iki byte uzunlugundaki isaretli integer
//     let harika_bir_i16 = -0xDE_i16;
//     // i32 - 4 byte uzunlugundaki isaretli integer
//     let bildigimiz_integer = 1337_i32;
//     // i64 - 8 byte uzunlugundaki isaretli integer
//     let i64_guzelligi = 0xDEADBEEF_i64;
//     // i128 - 16 byte - WOW - isaretli integer
//     let sisman_integer = 0xDEADBEEF_CAFEBABE_i128;
//     let sisman_integer_normal_sayi_yazayim: i128 = 12837243589723986529856;
//
//     // Unsigned Integer Duklugu
//
//     // Yukaridakilerin aynisi ama bu sefer `u` kullaniyoruz. Isaretsiz integerlar.
//     // u8
//     let bir_u8: u8 = 3_u8;
//     // u16 - iki byte uzunlugundaki isaretsiz integer
//     let harika_bir_u16 = 0xDE_u16;
//     // u32 - 4 byte uzunlugundaki isaretsiz integer
//     let bildigimiz_integer = 1337_u32;
//     // Ve digerleri
//
//     // Floating Point Banliyosu.
//     // Virgullu sayilar. Mantissa + Exponent IEEE standardindadir.
//
//     // Float 4 bytelik yer kaplar. Rust float'i f32 dir.
//     let virgullu_sayi_float: f32 = 123.143_f32;
//     let pi_sayisi: f32 = std::f32::consts::PI;
//     let epsilon_sayisi_euler: f32 = std::f32::consts::E;
//
//     // Double 8 bytelik yer kaplar. Rust double'i f64 tur.
//     let virgullu_sayi_double: f64 = 9818237.143_f64;
//     let pi_sayisi: f64 = std::f64::consts::PI;
//     let epsilon_sayisi_euler: f64 = std::f64::consts::E;
//
//
//     // String Federasyonu
//
//     let stack_uzerinde_guzel_bir_string: &str = "🥙 Doner seviyorum.";
//     let yemek_guzeldir: &str = r#"
//     |Efsane yiyecekler listesi:
//     |    1. Iskender 🍢
//     |    2. Ali Nazik 🥘
//     |    3. Spaghetti Carbonara 🍝
//     "#;
//
//     println!("{stack_uzerinde_guzel_bir_string}");
//     println!("{yemek_guzeldir}");
//
//     // Char Voyvodaligi
//     // char veri tipi 32 bit uzunlugundadir.
//
//     let sonsuz_bir_char: char = '∞';
//     let tek_karakter: char = 'a';
//     // Not: String iteratorleri char verir.
//
//     // Byte String Cumhuriyeti
//     let byte_array: &[u8; 10] = b"cannonball";
//     let kasaba_tarihi: &[u8; 97] = br#"
//         Cannonball, Amerika Birlesik Devletleri'nde
//         bulunan kurgusal bir kasabadir.
//     "#;
//     // Farkindaysaniz kasaba_tarihi degiskeninde UTF-8 karakter kullanmadik.
//     // Bytearrayler yazilirken UTF-8 karakter kabul etmezler.
//     // Bytearrayler ASCII karakter kabul eder.
//
//
//     // Boolean Arsiduklugu
//     // Booleanlar 8 bit uzunlugundadir.
//     let dogruyum: bool = true;
//     let yanlisim: bool = false;
// }

// ✨ Asama 5. Veri tipleri: Compound - Birlesik veri tipleri

// fn main() {
//     // Arrays - Diziler
//     let mut on_tane_42: [i8; 10] = [42; 10]; // Deger degistirecegimiz icin `mut`.
//     on_tane_42[2] = 0;
//     on_tane_42[5] = -12;
//     println!("on_tane_42'nin son hali: {:?}", on_tane_42);
//
//     let initialize_edilen_array: [i8; 3] = [20, 30, 40];
//     let tipi_infer_edilen_array = [12, 43, 54_i8];
//
//     dbg!(&initialize_edilen_array);
//     dbg!(&tipi_infer_edilen_array);
//
//     // Tuples - Tuplelar
//     let t: (i8, bool) = (7, true);
//     println!("Birinci: {}", t.0);
//     println!("Ikinci: {}", t.1);
//
//     let (sentaks, sekerleri) = (1234, "CannonBall Kasabasi");
//     dbg!(&sentaks, &sekerleri);
// }

// ✨ Asama 6. Referanslar

// fn main() {
//     let mut x: i32 = 10;
//     // mutable borrow - Degistirilebilmeyi saglayan odunc alma.
//     let ref_x: &mut i32 = &mut x;
//     *ref_x = 20;
//     println!("x: {x}");
// }

// ✨✨ Asama 7. Referanslar - Bosta olan Referanslar yoktur.
// Derleyici sizi bosta kalan referanslara karsi korur.

// fn main() {
//     // SCOPE B
//     let ref_x: &i32;
//
//     { // SCOPE A
//         let x: i32 = 10;
//         ref_x = &x; // Buna `borrow` - `odunc alma` denir.
//         println!("ref_x in scope A: {ref_x}");
//     }
//
//     // println!("ref_x in scope B: {ref_x}");
// }

// ✨ Asama 8. Slices - Dilimler

// fn main() {
//     // Basit bir array.
//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     println!("a: {a:?}");
//
//     // Sadece 2 ve 4 arasini alan bir slice.
//     let s: &[i32] = &a[2..4];
//     println!("s: {s:?}");
//
//     // Soru: Tam buraya a[3] = 99; yazarsam ne olur.
//
//     // Sadece 2 ve 4 arasini 5. element ile alan bir slice.
//     let f: &[i32] = &a[2..=4];
//     println!("f: {f:?}");
//
//     // Tum araligi alan bir slice.
//     let x: &[i32] = &a[..];
//     println!("x: {x:?}");
// }

// ✨ Asama 9. `String` ve `str` arasindaki fark.

// fn main() {
//     // `&str` immutable - degistirilemez string cesididir.
//     // Sigiyorsa stackte sigmiyorsa heaptedir.
//     let s1: &str = "World";
//     println!("s1: {s1}");
//
//     // `String` mutable - degistirilebilen string cesididir.
//     // Buyuyup kuculebilen bir yapisi vardir.
//     let mut s2: String = String::from("Hello ");
//     println!("s2: {s2}");
//     s2.push_str(s1);
//     println!("s2: {s2}");
// }

// ✨ Asama 10. Fonksiyonlar
// Bir Fizz Buzz programi asagidaki gibi yazilir.
// Fizz Buzz programi verilen sayiya kadar:
// Sayi 3e bolundugunde `fizz`,
// Sayi 5e bolundugunde `buzz`,
// Sayi hem 3e hem 5e bolundugunde `fizzbuzz`,
// Eger sayi ne 3e ne 5e bolunmuyorsa o zaman da
// sayinin kendisini yazmasi gerekir.

// fn main() {
//     fizzbuzz_yap(20);   // Asagida tanimlandigi icin onceden deklare etmeye gerek yok.
// }
//
// fn bolunur_mu(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false;  // 0 ile bolmeyi onlemek icin return
//     }
//     lhs % rhs == 0     // boluntu kontrolu
// }
//
// fn fizzbuzz(n: u32) -> () {  // Bir sey dondurmuyorsak `()` birim tip donduruyoruz.
//     match (bolunur_mu(n, 3), bolunur_mu(n, 5)) {
//         (true,  true)  => println!("fizzbuzz"),
//         (true,  false) => println!("fizz"),
//         (false, true)  => println!("buzz"),
//         (false, false) => println!("{n}"),
//     }
// }
//
// fn fizzbuzz_yap(n: u32) {  // `-> ()` normalde atilir. Gereksinim yoktur.,
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// }

// ✨ Asama 11. Metodlar

// struct Dikdortgen {
//     genislik: u32,
//     yukseklik: u32,
// }
//
// // `impl` ile belirli tipe iliskin metodlar yazilir.
// impl Dikdortgen {
//     fn alan(&self) -> u32 {
//         self.genislik * self.yukseklik
//     }
//
//     fn genislik_arttir(&mut self, ekstra: u32) {
//         self.genislik += ekstra;
//     }
// }
//
// fn main() {
//     let mut ddgen = Dikdortgen { genislik: 10, yukseklik: 5 };
//     println!("Eski Alan: {}", ddgen.alan());
//     ddgen.genislik_arttir(5);
//     println!("Yeni Alan: {}", ddgen.alan());
// }