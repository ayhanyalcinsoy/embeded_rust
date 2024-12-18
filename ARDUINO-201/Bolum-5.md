# 5. Millis Fonksiyonu Nedir ve Nerelerde Kullanılır?

Millis fonksiyonu Arduino'ya enerji verildiği zaman otomatik olarak çalışmaya başlar. Fonksiyon içerisinde Arduino'nun çalışma zamanı milisaniye cinsinden tutulur. Bu fonksiyon çağrıldığında Arduino'nun kaç milisaniye süresince çalıştığı döndürülür. Değerler milisaniye cinsinden tutulduğu için geri döndürülen değer int veya float'ın tutabileceği kapasitenin çok üstünde olabilir. Bu yüzden bu fonksiyonla birlikte unsigned long türündeki değişkenler kullanılır.

Not: Millis fonksiyonunu yaklaşık 50 gün çalışabilmektedir. Bu süreden sonra kendisini sıfırlar. Arduino'yu 50 gün boyunca hiç kapatmadan çalıştırdığımızı düşünelim. 50. günün sonunda millis fonksiyonunu kullandığımızda, fonksiyon sıfırlanacağı için Arduino yeniden başlamış gibi değerler döndürecektir.

Millis fonksiyonunun kullanımı gecikme için kullanılan delay fonksiyonunun kullanımına benzemektedir fakat delay fonksiyonu kullanıldığında Arduino başka bir işlem yapamamaktadır. Örneğin saniyede bir LED yakıp söndürmek isteyelim. Bu işlemi aşağıdaki komutla çok kolay bir şekilde yapabiliriz.

```cpp
void setup() {
  pinMode(13,OUTPUT);
}
void loop() {
  digitalWrite(13,HIGH);
  delay(1000);
  digitalWrite(13,LOW);
  delay(1000);
}
```
Yukarıdaki kod denendiğinde gerçekten de saniyede bir yanan ve sönen LED elde etmiş oluruz. Bu sırada LED'i yakıp söndürmekten başka bir iş yapamayız, çünkü delay fonksiyonu Arduino'yu bir saniye süresince bekletmektedir. Eğer bir saniyede bir LED yakıp söndürmek ve bu arada da seri monitöre sürekli olarak bir şeyler yazdırmak isteseydik, burada delay fonksiyonu kullanamazdık. İşte bu noktada millis fonksiyonunun kullanılması gerekmektedir.

## 5.1. Delay fonksiyonu yerine millis fonksiyonunun kullanılması

Aşağıdaki kodla hem saniyede bir LED yakıp söndürülmüş hem de sürekli olarak serial monitöre "Arduino burada başka işlemler de yapabilir." mesajı yazdırılmıştır.

```cpp
void setup() {
  pinMode(13,OUTPUT);
  Serial.begin(9600);
}

/* zaman değerlerinin tutulması için değişkenler tanımlanmalı */
unsigned long eskiZaman=0;
unsigned long yeniZaman;

int LEDdurumu = 0;

void loop() {
  /* Arduinonun çalışma suresi milisaniye cinsinden alınıyor */
  yeniZaman = millis(); 
  /* bir önceki turdan itibaren 1000 milisaniye geçmiş mi
  yani yeniZaman ile eskiZaman farkı 1000den büyük mü */
  if(yeniZaman-eskiZaman > 1000){
     if(LEDdurumu == 1){
      digitalWrite(13,LOW);
      LEDdurumu = 0;
     }else{
      digitalWrite(13,HIGH);
      LEDdurumu = 1;
     } 
     /* Eski zaman değeri yeni zaman değeri ile güncelleniyor */
     eskiZaman = yeniZaman;
  }
  
  /*Ekrana sürekli olarak mesaj yazdırılıyor */
  Serial.println("Arduino burada baska islemler de yapabilir.");
  /* Bekleme fonksiyonunun kullanılmasına gerek yok
  fakat ekrana çok hızlı yazdırmamak için biraz bekleme kullanıyoruz */
  delay(10);
}
```

Yukarıdaki kodda setup fonksiyonu içerisinde LED pini çıkış yapılmış ve seri haberleşme başlatılmıştır. Daha sonra millis fonksiyonunun döndürdüğü değerlerin tutulması için unsigned long tipinde değişkenler tanımlanmıştır. "yeniZaman" değişkeni millis fonksiyonundan dönecek sayıları tutmak için kullanılmıştır. "eskiZaman" fonksiyonu ise önceki süreyi tutmaktadır.

Bin salise yani bir saniyenin geçip geçmediğini kontrol etmek için "yeniZaman" değişkeninden "eskiZaman" değişkeni çıkartılmıştır. Eğer bu çıkartma işleminin değeri 1000 değerinden fazlaysa bir saniyenin geçtiği anlaşılmaktadır. Bu işlemlerin dışında da ekrana sürekli olarak mesaj yazdırılmaktadır. Böylece delay fonksiyonu yerine millis fonksiyonunun kullanılırsa, bu sürede başka işlemlerin de yapılabileceğini görmüş olduk.

## 5.2. Performans testlerinde millis fonksiyonunun kullanımı

Millis fonksiyonu Arduino'nun çalışma performansını ölçmek için de kullanılır. Örneğin, çok fazla işlem yapan bir fonksiyonumuz olsun. Bu fonksiyon her çağrıldığında Arduino'nun bu işlemleri yapabilmesi için belirli bir süre geçmesi gerekmektedir. Bu zaman kayması da bizim diğer işlerimizi aksatabilir. Bu fonksiyonun her çağrıldığında ne kadar zaman kullandığını öğrenmek için millis fonksiyonu yardımıyla aşağıdaki gibi bir test kodu yazalım.

```cpp
void setup() {
  Serial.begin(9600);
}

void A(){
  /* fonksiyon icerisinde zaman alcak islemler yapilmistir*/
  unsigned long sonuc=0;
  for(int i = 0; i < 999; i ++){
      sonuc += i;
      delay(1);
  }
  Serial.print("A fonksiyonunun cevabi =");
  Serial.println(sonuc);
  Serial.println();
}

/* zaman değerlerinin tutulması için değişkenler tanımlanmalı */
unsigned long zamanbir=0;
unsigned long zamaniki=0;

void loop() {
  /* Arduinonun çalışma suresi milisaniye cinsinden alınıyor */
  zamanbir = millis();
 /* A fonksiyonu çağırılıyor */ 
  A();
  zamaniki = millis();
  /* A fonksiyonunun çalışma süresi zamaniki-zamanbir denklemi ile hesaplanıyor */
  Serial.print("A fonksiyonunun calisma suresi= ");
  Serial.print(zamaniki-zamanbir);
  Serial.println(" milisaniyedir.");
}
```

Yukarıdaki kodda setup fonksiyonu içerisinde seri monitöre yazdırmak için her zamanki gibi seri haberleşme başlatılmıştır. Daha sonra test etmek istediğimiz fonksiyon yani A fonksiyonu tanımlanmıştır. Bu fonksiyonun içeriği önemli değildir, sadece zaman alacak bir fonksiyon yazılmıştır. Millis fonksiyonunun döndürdüğü değerleri tutmak için bir önceki uygulamamızdaki gibi iki adet unsigned long türünde değişken tanımlanmıştır.

Loop fonksiyonu içerisinde öncelikle millis fonksiyonuyla Arduino'nun o anki çalışma süresi milisaniye cinsinden zaman bir değişkenine kaydedilmiştir. Kayıt işlemi yapıldıktan sonra A fonksiyonu çağrılmıştır. Bu fonksiyon tamamlandıktan sonra ne kadar süre geçtiğini anlamak için zamaniki değişkenine Arduino'nun o anki çalışma süresi yazdırılmıştır. A fonksiyonunun toplam kullandığı zamanı hesaplamak için ikinci ölçümden birinci ölçüm çıkartılmış ve ekrana yazdırılmıştır.