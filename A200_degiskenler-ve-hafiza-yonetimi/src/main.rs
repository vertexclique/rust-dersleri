// ✨ Asama 12. Degiskenler

// Rust tip guvenligini static typing ile saglar.
// Degisken tanimlamalari `immutable` dir.
// Degiskene degisken diyoruz ama degismiyor.
// Terminolojinin gazabina ugruyoruz.

// fn main() {
//     let x: i32 = 10;
//     println!("x: {x}");
//
//     // Asagidaki kod blogunu uncomment edelim.
//
//     // x = 20;
//     // println!("x: {x}");
// }

// ✨ Asama 13. Veri Tipi Cikarimi Yapimi

// Rust Hindley-Milner tip sistemini kullanir.
// An itibariyle derleyici'nin kullandigi tip cikarimi
// `W algoritmasinin` bir versiyonudur.
// Ayrica, lexical scope'ta bulunan degiskenler (asagidaki main kodunun ici)
// ilk once derleyicide tanimlanir sonra gelen tum uyusmazliklar ise derleyici
// hatasi olarak size yansir.
// Bu tip cikarimini anlamak icin asagidaki kodu uncomment edin.

// fn u32_alirim(x: u32) {
//     println!("u32: {x}");
// }
//
// fn i8_alirim(y: i8) {
//     println!("i8: {y}");
// }
//
// fn main() {
//     let x = 10;
//     let y = 20;
//
//     u32_alirim(x);
//     i8_alirim(y);
//     // u32_alirim(y);
// }

// ✨ Asama 14. Statik ve Sabit Degiskenler

// Rust'in sabit sistemi cok gelismistir.
// Bir programda ne kadar fazla sabit varsa o program daha linearize sekilde calisir.
// Bu daha az bounds checking, accumulator load ve ekstra check kodu ve dolayisiyla
// daha az assembly kodunun derleyiciden ejekte edilmesine imkan sunar.

// Global durum statik ve sabit degiskenler ile yonetilir.
// Hatirladiniz mi? Suslu parantez scope degil demistik.
// Global scope, module scopes, function scopes, impl scopes ve daha bir suru scope Rust'ta vardir.
// Simdi global scope'tan bahsediyoruz.
// Tum programa acik olan scope'tan.\
// XXX: const ifadeler ile alakali diger konulara Advanced Rust dersinde deginecegiz.

// Hadi derleyici zamani sabitleri ogrenelim:

// const DIGEST_BUYUKLUGU: usize = 3;
// const SIFIR: Option<u8> = Some(42);
//
// fn digest_hesapla(veri: &str) -> [u8; DIGEST_BUYUKLUGU] {
//     let mut digest = [SIFIR.unwrap_or(0); DIGEST_BUYUKLUGU];
//     for (idx, &b) in veri.as_bytes().iter().enumerate() {
//         digest[idx % DIGEST_BUYUKLUGU] = digest[idx % DIGEST_BUYUKLUGU].wrapping_add(b);
//     }
//     digest
// }
//
// fn main() {
//     let digest = digest_hesapla("Hello");
//     println!("Hesaplanan digest: {digest:?}");
// }

// Static degiskenleri de ogrenelim.
// Static degiskenler degistirilebilir ama sabitler asla degistirilmez.

// static GUNUN_MESAJI: &str = "Rust Derslerine Hosgeldiniz! Theo Hoca ile kaydiriyoruz!";
//
// fn main() {
//     println!("{GUNUN_MESAJI}");
// }

// ✨ Asama 14. Lexical Scopelar ve Shadowing (Golgeleme)
