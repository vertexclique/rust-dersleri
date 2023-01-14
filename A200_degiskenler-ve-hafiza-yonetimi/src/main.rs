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

// fn main() {
//     let a = 10;
//     println!("en basta: {a}");
//
//     {
//         let a = "hello";
//         println!("ic scope: {a}");
//
//         let a = true;
//         println!("shadowlanmis ic scopetaki a: {a}");
//     }
//
//     println!("dis scopeta a: {a}");
// }

// ✨✨ Asama 15. Hafiza Yonetimi

// Programlama dilleri hafiza yonetimi bakimindan ikiye ayrilir:
// 1. Manuel hafiza yonetimi ile full kontrol verenler: C, C++, Pascal.
// 2. Otomatik hafiza yonetimi ile guvenlik verenler: Java, Python, Go, Haskell.

// Rust ise bambaska bir dildir. Hem kontrol hem guvenligi derleme zamaninda dogru sekilde hafiza yonetimini
// program yazan icin zorlayarak yeni bir bakis acisi getirir.
// Bunu ownership (sahiplik) sistemiyle yapar.

// Baslamadan once gelin stack ve heap kavramlarina bir bakis atalim.

// Stack: Yerel değişkenler için sürekli bellek alanı.
// * Değerlerin derleme zamanında bilinen sabit boyutları vardır.
// * Son derece hızlı: sadece bir stack pointeri tasimak gerekir.
// * Yönetimi kolaydır: fonksiyon çağrılarını takip eder.
// * Mükemmel bellek yerelliği(locality).
// * Asagiya dogru buyur.
//
// Heap: Fonksiyon çağrıları dışında değerlerin depolanması.
// * Değerler çalışma zamanında belirlenen dinamik boyutlara sahiptir.
// * Yığından biraz daha yavaştır: biraz bookkeeping isi gerekir.
// * Bellek yerelliği garantisi yoktur. Dipnot: Nedenini isterseniz aciklayabilirim.
// * Yukari dogru buyur.


// ✨✨ Asama 16. Stack (Yigin) Hafiza
// String oluşturmak sabit boyutlu verileri yığına, dinamik boyutlu verileri ise heap'e yerleştirir:

fn main() {
    let s1 = String::from("Hello");
}

// ```bob
//  Stack                             Heap
// .- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - -.
// :                           :     :                               :
// :    s1                     :     :                               :
// :   +-----------+-------+   :     :   +----+----+----+----+----+  :
// :   | ptr       |   o---+---+-----+-->| H  | e  | l  | l  | o  |  :
// :   | len       |     5 |   :     :   +----+----+----+----+----+  :
// :   | capacity  |     5 |   :     :                               :
// :   +-----------+-------+   :     :                               :
// :                           :     `- - - - - - - - - - - - - - - -'
// `- - - - - - - - - - - - - -'
// ```
//
// `String`in arkasinda `Vec` dedigimiz genisleyip kuculebilen ve bu genislemeyi heap uzerinde saglayabilen bir veri yapisi bulunur.
// Bu heap uzerinde hafiza alimini saglayan sey ise `Sistem Hafiza Ayiricisi` dedigimiz Rust'in varsayilan olarak kullandigi hafiza ayiricisidir. 
// Kendiniz hafiza ayiricisi gelistirebilir ve Rust'a baska hafiza ayiricilari atayabilirsiniz. Hafiza ayiricilarinin spesifik ozelliklerini "Advanced Rust" dersinde gosterecegiz.

// ✨✨ Asama 17. Manuel Hafiza Yonetimi

// Hafiza kullanan bir C kodu yazdigimizda hafiza alanini kendimiz ayirip kendimiz isletim sistemine geri veririz.
// C programlarinda hafizayi geri verme islemini `free` fonksiyonu, hafiza ayirma islemini ise `malloc` fonksiyonu yapar.
// 
// ```c
// void foo(size_t n) {
//     int* int_array = (int*)malloc(n * sizeof(int));
//     //
//     // ... lots of code
//     //
//     free(int_array);
// }
// ```
//
// Fonksiyon `malloc` ve `free` arasında erken dönerse bellek sızdırılır: işaretçi kaybolur ve belleği ayıramayız.

// ✨✨ Asama 18. Scope Tabanli Hafiza Yonetimi

// Kurucular ve yıkıcılar, bir nesnenin yaşam süresine bağlanmanızı sağlar.
//
// Bir işaretçiyi bir nesneye sararak, nesne yok edildiğinde belleği boşaltabilirsiniz.
// Derleyici, bir istisna ortaya çıksa bile bunun gerçekleşeceğini garanti eder.
//
// Bu genellikle kaynak edinimi başlatma (RAII) olarak adlandırılır ve size akıllı işaretçiler sağlar.
//
// Scope tabanli hafiza yonetiminde ise isler farkli isler. Bu C++'in hafiza yonetim sistemidir.
//
// ```c++
// void merhaba_de(std::unique_ptr<Insan> insan) {
//     std::cout << "Merhaba " << insan->isim << std::endl;
// }
// ```
//
// * `std::unique_ptr` nesnesi stackte ayrilir ve heap üzerinde ayrilan belleğe işaret eder.
// * `merhaba_de`'nin sonunda, `std::unique_ptr` yıkıcısı çalışacaktır.
// * Yıkıcı, işaret ettiği `Insan` nesnesini serbest bırakır.
//
// Bir fonksiyona sahiplik aktarılırken özel move kurucuları kullanılır:
//
// ```c++
// std::unique_ptr<Insan> insan = insan_bul("Mesude");
// merhaba_de(std::move(insan));
// ```

// ✨✨ Asama 19. Otomatik Hafiza Yonetimi - Garbage Collection (Cop Toplama)

// Scope tabanli ve manuel hafiza yonetimine alternatif olan bu yontemde:
// * Programci acikca hafizayi kendisi ayirip, geri vermez.
// * Bir cop toplayıcı kullanılmayan belleği bulur ve programcı için ayırır.
//
// ```java
// void merhabaDe(Insan insan) {
//     System.out.println("Merhaba " + insan.getIsim());
// }  
// ```

// ✨✨ Asama 20. Rust'ta Hafiza Yonetimi

// Rust'ta hafiza yonetimi ise bunlarin bir mixidir:
// * Java gibi güvenli ve doğrudur, ancak çöp toplayıcısı yoktur.
// * Hangi soyutlamayı (veya soyutlama kombinasyonunu) seçtiğinize bağlı olarak,
// tek bir benzersiz işaretçi, referans sayımlı veya atomik referans sayımlı olabilir.
// * C++ gibi scope tabanlıdır, ancak derleyici tam scope tabanli hafiza yonetimini zorunlu kılar.
// * Bir Rust kullanıcısı durum için doğru soyutlamayı seçebilir (ZCA - zero-cost abstractions),
// hatta bazılarının C gibi çalışma zamanında hiçbir maliyeti yoktur.

// Tum bunlarin hepsi bir sonraki konumuz olan `Ownership` (Sahiplik) modeli ile yapilir.
// Soru: Hocam bu dediginiz seyleri Rust neler ile yapiyor?
// Yanit: Rust'in kendine ait RAII (Resource Acquisition Is Initialization - kaynak edinimi başlatma)
// bunlar, Box, Vec, Rc, veya Arc tir.
//
// Soru: C++'in yikicilarinin benzeri Rust'ta var mi?
// Yanit: Evet var, `Drop` traiti bu isi gorur.

// ✨ Asama 21. Hafiza yonetim sistemlerinin bir karsilastirmasi

// ++++++ Farklı Bellek Yönetimi Tekniklerinin Artıları ++++++
//
// C gibi manuel:
// * Çalışma zamanı ek yükü yok.
//
// Java gibi otomatik:
// * Tam otomatik.
// * Güvenli ve doğru.
//
// C++ gibi scope tabanlı:
// * Kısmen otomatik.
// * Çalışma zamanı ek yükü yok.
// 
// Rust gibi derleyici tarafından zorlanan scope tabanlı:
// * Derleyici tarafından zorlanır.
// * Çalışma zamanı ek yükü yok.
// * Güvenli ve doğru.

// ------ Farklı Bellek Yönetimi Tekniklerinin Eksileri ------
//
// C gibi manuel:
// * Use-after-free.
// * Double-frees.
// * Bellek sızıntıları.
//
// Java gibi otomatik:
// * Çöp toplama duraklamaları.
// * Yıkıcı gecikmeleri.
//
// C++ gibi scope tabanlı:
// * Karmaşık, programcı tarafından tercih edilir.
// * Use-after-free (kullanım sonrası serbest bırakma) potansiyeli.
// 
// Rust gibi derleyici zorlamalı ve kapsam tabanlı:
// * Bazı ön karmaşıklık ve kompleksite.
// * Dogru programları reddedebilir.