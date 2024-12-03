# TEMEL ELEKTRONİK VE ROBOTİĞE GİRİŞ

# 1. ROBOTİK
## 1.1 Robotik Nedir?

Robotik, Yapay Zekanın (AI) bir dalıdır, esas olarak inşaat, tasarım ve tasarım için elektrik mühendisliği,
makine mühendisliği ve bilgisayar bilimi mühendisliğinden oluşur.

Robotik, robotların bir uygulamasını inşa etme veya tasarlama bilimidir. Robotiğin amacı verimli bir robot
tasarlamaktır.

**Robotiğin Yönleri**

 * Robotlar, güç sağlamak ve makineyi kontrol etmek için elektrikli bileşenlere sahiptir.
 * Belirli bir görevi yerine getirmek için tasarlanmış mekanik yapıya, şekle veya forma sahiptirler .
 * Bir robotun neyi, ne zaman ve nasıl yapacağını belirleyen bir tür bilgisayar programı içerir .

## 1.2.Robotik Tarihi

**Robotik kelimesinin ilk kullanımı:**
Robot kelimesi ilk olarak Çek yazar Karel Çapek tarafından 1920 yılında yayınlanan Rossum'un Evrensel
Robotları (RUR) adlı oyunuyla kamuoyuna tanıtıldı. Oyun, robot olarak bilinen yapay insanları yapan bir
fabrika ile başlar.

"Robotik" kelimesi, 1940'lı yıllarda Rus asıllı Amerikalı bilim adamı Issac Asimov tarafından tesadüfen icat
edildi.

**Robotiğin üç yasası:**

Issac Asimov ayrıca üç "Robot Yasasını" önerdi ve daha sonra bir "sıfırcı yasa" ekledi.

**Sıfırıncı Yasa** - Bir robotun insanlığı incitmesine izin verilmez veya eylemsizlik yoluyla insanlığın zarar
görmesine izin verilir.

**Birinci Yasa** - Bir robot, bir insanı yaralayamaz veya daha yüksek bir yasayı ihlal etmedikçe, bir
insanın zarar görmesine izin veremez.

**İkinci Yasa** - Bir robot, insanlar tarafından verilen bu tür emirlerin daha yüksek bir kanunla çelişmediği
durumlar dışında, insanlar tarafından verilen emirleri yerine getirmelidir. Üçüncü Yasa - Bir robotun
kendi varlığını korumasına, bu tür bir koruma daha yüksek bir kanunla çelişmediği sürece izin verilir.

**Üçüncü Kanun** - Bir robotun kendi varlığını korumasına, bu tür bir koruma daha yüksek bir kanunla
çelişmediği sürece izin verilir.

**İlk endüstriyel robot: UNIMATE**
1954'te ilk programlanabilir robot, Evrensel Otomasyon terimini icat eden George Devol tarafından tasarlandı.
Daha sonra bu terimi 1962'de ilk robot şirketinin adı haline gelen Unimation olarak kısaltır.

![Unimate](../images/robot_unimate.webp)

## 1.3.Roboton Bileşenleri
Bir robot elektrik, elektronik, mekanik parçalar ve yazılımın sistemli ve düzenli bir şekilde bir araya getirilmesiyle oluşturulmaktadır. Aşağıda bir robota ait bileşenler gösterilmektedir.

![Robot Bileşenleri](../images/robot_parts.png)

Şematik olarak gösterilirse bir robotun anatomisi (yapısı)aşağıdaki gibidir:

![Robot Anatomisi](../images/components-of-robot2.png)

 * **Güç Kaynağı** - Robotun çalışma gücü piller, hidrolik, güneş enerjisi veya pnömatik güç kaynakları
tarafından sağlanır.

 * **Aktüatörler** - Aktüatörler, bir robotun içinde kullanılan enerji dönüştürme cihazıdır. Aktüatörlerin ana
işlevi, enerjiyi harekete dönüştürmektir.

 * **Elektrik motorları (DC/AC)**- Motorlar, elektrik enerjisini eşdeğer mekanik enerjiye dönüştürmek için
kullanılan elektromekanik bileşenlerdir. Robotlarda dönme hareketini sağlamak için motorlar kullanılmaktadır.

 * **Sensörler** - Sensörler, görev ortamı hakkında gerçek zamanlı bilgi sağlar. Robotlar, insan parmak
izlerinin dokunma reseptörlerinin mekanik özelliklerini taklit eden dokunsal sensörle donatılmıştır ve
ortamdaki derinliği hesaplamak için bir görüş sensörü kullanılır.

## 1.4.Robot Türleri

### 1.4.1. Mobil Robotlar

Mobil robotlar, hareket kabiliyetini kullanarak bir konumdan başka bir konuma hareket edebilir. Herhangi bir
fiziksel ve elektromekanik yönlendirme cihazına ihtiyaç duymadan kontrolsüz bir ortamda seyir yapabilen
otomatik bir makinedir. Mobil Robotlar iki tiptir:

 **Yuvarlanan robotlar** - Yuvarlanan robotların hareket etmesi için tekerleklere ihtiyacı vardır. Kolay ve
hızlı arama yapabilirler. Ancak yalnızca düz alanlarda kullanışlıdırlar.

![Yuvarlanan Robotlar](../images/types-of-robot1.png)

 **Yürüyen robotlar** - Ayaklı robotlar genellikle arazinin kayalık olduğu durumlarda kullanılır. Çoğu
yürüyen robotun en az 4 ayağı vardır.

![Yürüyen Robotlar](../images/types-of-robot2.png)

### 1.4.2. Endüstriyel Robotlar

Endüstriyel robotlar, hiç hareket etmeden aynı görevleri tekrar tekrar gerçekleştirir. Bu robotlar, robota uygun
sıkıcı ve tekrarlanan görevlerin yapılmasının gerekli olduğu endüstrilerde çalışmaktadır.
Bir endüstriyel robot asla yorulmaz, gece gündüz hiç şikayet etmeden işlerini yapar.

![Endüstriyel Robotlar](../images/types-of-robot3.png)


### 1.4.3. Ontonom Robotlar

Otonom robotlar kendi kendini destekler. Çevrelerine bağlı olarak gerçekleştirecekleri eyleme karar verme
fırsatı sağlayan bir program kullanırlar.

Yapay zekayı kullanan bu robotlar genellikle yeni davranışlar öğrenir. Kısa bir rutinle başlarlar ve
gerçekleştirdikleri bir görevde daha başarılı olmak için bu rutini adapte ederler. Bu nedenle, en başarılı rutin
tekrarlanacaktır.

![Otonom Robotlar](../images/types-of-robot4.png)

### 1.4.4. Uzaktan Kumandalı Robotlar

Uzaktan kumandalı robot, operasyon belirsizliği nedeniyle otonom robotun yapamadığı karmaşık ve belirsiz
görevleri gerçekleştirmek için kullanılır.

Karmaşık görevler, gerçek beyin gücüne sahip insanlar tarafından en iyi şekilde gerçekleştirilir. Bu nedenle,
bir kişi uzaktan kumandayı kullanarak bir robotu yönlendirebilir. İnsan, uzaktan kumandalı çalışmayı
kullanarak, görevlerin gerçekleştirildiği noktada bulunmadan tehlikeli görevleri gerçekleştirebilir.

Uzaktan kumandayla tasarlanmış bir NASA robotu görelim:

![Uzaktan Kumandalı Robotlar](../images/types-of-robot5.png)


# 2. ELEKTRİK VE ELEKTRONİK

Arduino, yazılım ve elektroniğin bir araya getirildiği ortamdır. Bu yüzden Arduino kullanmaya başlamadan önce temel elektronik bilgilerimizi tazelemeliyiz. Bu bölümde temel elektronik devre elemanlarını tanıyacağız ve bu elemanların nasıl kullanıldığını öğreneceğiz.

Elektronik elektriğe yön verme sanatıdır.Bu yönde oluşturulan devreler örnek vermek gerekirse harekete
duyarlı bir lamba için kullanılabilmektedir.

Diğer bir örnek ise otomatik açılan kapıları verebiliriz.Bu yapının oluşması için elektrik, devre, sensör,
elektronik, elektronik malzeme ve cihazlar kullanılmaktadır.

## 2.1. Elektrik Nedir?

Elektrik, elektrik yüklerinin akışına dayanan bir dizi fiziksel olaya verilen isimdir. Elektrik sözcüğü Türkçeye
Fransızcadan geçmiştir. Elektriğin Türkçe eş anlamlısı **çıngı** sözcüğüdür. Ayrıca Anadolu'da **ceryan** olarak
söylenmektedir.

Elektrik gözle görünmez, ama etki ettiği cihazlar üzerinden görme şansımız vardır.Bunlara örnek vermemiz
gerekirse, lamba yanması, evimizde çalışan beyaz eşyalar ve küçük ev aletleri elektriğin varlığını bize
göstermektedir.

Yukarıdaki örnekler elektriğin neden olduğu, ışık, ısı, ses ve hareket gibi fiziksel etkenleri görmekteyiz.Aynı
zamanda elektrik elde etmek içinde su ve güneş gibi unsurlarıda kullanmaktayız.

|                                                   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|---------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|![Atom Yapısı](../images/atomun-yapisi-300x212.jpg)|Elektiğin ne olduğunu daha iyi anlamak için biraz detaylandıralım, maddenin yapı taşı olan **atom** boyutuna bakmamız gerekiyor, atom kendinden daha küçük üç yapı taşından oluşur, bunlar; nötron, proton ve elektron'dur.Bu yapı taşları atom çekirdeğinde bulunur. Elektronlar (- eksi) yük, protonlar ise (+ artı) yüklü iken, **nötronlar** ise yüksüzdür.Zıt yükler birbirini çekerler. Buradan yola çıkarak yüklü elektron parçacıklarının hareketlerine **elektrik** diyoruz.Elektiriği ise kablolar yardımı ile taşımaktayız.|

Elektiriği daha iyi anlamak için **gerilim** ve **akım** kavramlarını anlamak gerekiyor.
Gerilim ya da voltaj elektronları maruz kaldıkları elektrostatik alan kuvvetine karşı hareket ettiren kuvvettir. Bir elektrik alanı içindeki iki nokta arasındaki potansiyel fark olarak da tarif edilir.Gerilimin birimi Volt, sembolü V dir.

Elektrik akımı, elektriksel akım veya cereyan, en kısa tanımıyla elektriksel yük taşıyan parçacıkların
hareketidir. Bu yük genellikle elektrik devrelerindeki kabloların içerisinde hareket eden elektronlar tarafından
taşınmaktadır. Akımın birimi **Amper**, sembolü **A** dır.

Alternatif akım, genliği ve yönü periyodik olarak değişen elektriksel akımdır. **AC** (Alternating current) olarak kısaltılır. +330 ile 0 arası ve 0 ile -330 volt arasında değişim olur, ölçü aletleri yapılan ölçümlerde 220V ölçülür. Evimizdeki elektrik alternatif akımdır.

Doğru akım elektrik yüklerinin yüksek potansiyelden alçak olana doğru sabit olarak akmasıdır. Tipik olarak
kablo gibi bir iletkende ya da yarı iletkenler ve yalıtkanlardan akabilir. Doğru akımda, elektrik yüklerinin aynı yönde akışı, doğru akımı alternatif akımdan ayırır. **DC** (Direct current) olarak kısaltılır. Batarya ve Pil buna örnektir.

## 2.2. İletken ve Yalıtkan

Maddeler elektrik akımını iletme durumlarına göre (Elektron hareketine göre) üçe ayrılabilir. Elektrik akımına
karşı çok küçük direnç gösteren malzemeler iletken, elektrik akımına karşı çok yüksek direnç gösteren
malzemeler yalıtkan olarak adlandırılabilir.

![İletken](../images/iletken_madde.jpg)


BURADA KALDIM


## 2.3. Dijital ve analog sinyaller

Sinyaller analog ve dijital olmak üzere ikiye ayrılır. Analog sinyaller devamlı sinyallerdir ve her değeri alabilirler. Örnek olarak Sinüs sinyali verilebilir. Dijital sinyaller ise devamlı değildir ve adım adım değişir. Örnek olarak PWM, kare dalgalar verilebilir. Arduino analog sinyalleri işleyemez, fakat doğadaki etkiler ve sensörler analog sinyal ile çalışır. Bu sinyallerin Arduino'da işlenebilmesi için dijital sinyale çevrilmesi gerekir. Bu çevirme işlemine analog dijital çevrim (ADC) denir.

Arduino'nun çıkış pinleri sadece 0 veya 5 volt verebilmektedir. Eğer bu pinlerden analog çıkış almak isterseniz, yani 0 veya 5 volt arasında, dijital analog çevrim (DAC) yapmalısınız. Bu özellikleri daha sonraki konularımızda daha detaylı olarak işleyeceğiz.

## 1.2. Breadboard

Breadboard, kullanacağımız elektronik elemanları bir arada tutmak ve gerekli kablo bağlantılarını gerçekleştirmek için kullanılır. Breadboard üzerinde iki çeşit yol vardır. Bunlardan ilki güç yollarıdır. Güç yolları, yani beslememizin artı ve eksi uçlarını taktığımız yer, resimde görülen kırmızı ve mavi şeritlerdir. Aşağıya doğru inen çizgilere karşılık gelen delikler kısa devre durumundadır. Bir başka deyişle, sol üstteki kırmızıdan bağlanan bir kablo aynı çizgi üzerinden bağlanacak kablolar ile birleşiktir. Aynı durum mavi çizgiler için de geçerlidir. Diğer elektronikçilerin de devrenizi anlayabilmesi için standartlara uygun olarak pilin artı ucu kırmızı çizgiye, eksi ucu ise mavi çizgiye takılmalıdır.

Diğer bir hatırlatma olarak da şunu belirtmekte fayda var. Bazı breadboardlarda yanlarda bulunan besleme hatları ikiye bölünmüş olduğu gibi, bazı breadboardlarda ise güç hatları tüm hat boyunca (yukarıdan aşağıya kadar) birbirine bağlıdır. Breadboard üzerindeki diğer yollar, güç hatlarının arasında bulunan yatay hatlardır. Bu hatlar yatay olarak birbirine bağlanmıştır. Fakat iki yatay hattı birbirinden ayırmak için arada bir boşluk vardır. Kısacası bu hatlar boşluğa kadar yatayda birbirine bağlıdır. Bu boşluğun amacı, elektronik entegrelerin takılabilmesini sağlamaktır.

![Breadboard](../images/breadboard-01.jpg)

Yukarıdaki görselde bir Breadboard'un iç yapısını görmektesiniz. Böylece Breadboard'daki deliklerin hangilerinin birbirine bağlı olduğunu anlayabilirsiniz.

## 1.3. Dirençler

Daha önce elektronikle çok az ilgilenmiş birinin bile bildiği direnç elemanı, hat üzerinden geçen akımı ayarlamak için kullanılır. V = İ * R formülünden de anlaşıldığı gibi sabit bir gerilime sahip hat üzerinden geçen akım azaltılmak isteniyorsa, direncin değeri yani R değeri artırılmalıdır. Aynı hat üzerinde bulunan elektronik elemanlar üzerinden geçen akımların birbirine eşit olmasından dolayı bu hat üzerinden geçen akımı kontrol etmek için uygun direnci kullanırız.

Örneğin, LED dediğimiz lambaların üzerinden fazla akım geçmesi bu lambalara zarar vermektedir. Bu lambaların fazla akım çekmesini engellemek için LED'in bağlantısından önce 220 ohm değerinde bir direnç takılır. Böylece LED üzerinden geçen akım azaltılmış olur. Eğer 220 ohm yerine daha büyük bir direnç bağlanırsa LED'in parlaklığında azalma olduğu görülür.

Direncin değeri ne yazık ki direnç üzerinde sayısal olarak yazmamaktadır. Fakat direncin değerinin anlaşılması için, direnç üzerinde renkli şeritler vardır. İlk iki şeritin değerleri ile iki haneli sayı oluşturulur. Bu iki haneli sayının da 103. şeridin değeri ile çarpılmasıyla direncin değeri bulunmuş olur.

Formül şu şekilde özetlenebilir:
Direncin değeri = ( 10x(ilk şeritin değeri) + 1x(ikinci şeritin değeri) )x10üçüncü şeridin değeri

** Renklerin Değerleri:**

|  Renk|  Değeri|  Renk    |  Değeri|  Renk | Değeri|  Renk | Değeri|  Renk| Değeri|
|------|--------|----------|--------|-------|-------|-------|-------|------|-------|
|Siyah |    0   |Kahverengi|    1   |Kırmızı|   2   |Turuncu|   3   |Sarı  |   4   |
|Yeşil |    5   |Mavi      |    6   |Mor    |   7   |Gri    |   8   |Beyaz |   9   |

Haydi, resimdeki direncin değerini hesaplayalım.

![Direnç](../images/elektrgiris_02.jpg)

Resimde dört adet şerit görülmektedir. Gümüş renkli şerit 4. şerittir. Bu şerit bize direncin toleransını göstermektedir. Direnç üzerindeki ilk şerit turuncu, ikinci şerit beyaz ve üçüncü şerit ise yeşildir. Yani ilk iki şeritin değeri 39'dur. Üçüncü şerit on üzeri şeklinde yazılırsa 105 yani 100.000 elde edilir. Bu sayıların çarpımı sonucunda direncin değeri 3.900.000 olarak hesaplanır. Kısaca direncin değeri 3,9M ohm'dur.

## 1.4. Voltaj Bölücü ve Potansiyometre

Voltaj Bölücü: Hattaki gerilimi daha düşük bir gerilime çevirmek için voltaj bölücü devresini kullanılır. Bu devrede iki tane direnç vardır. Kullanılan dirençlerin değerine göre çıkış gerilimi değişir. Voltaj bölücünün çıkışı besleme kaynağı olarak kullanılmamalıdır. Çünkü çıkıştaki elemanların iç direnci, voltaj bölücünün çıkış gerilimini de değiştirmektedir.

![Voltaj Bölücü](../images/elektrgiris_03.jpg)

Resimde voltaj bölücü devresinin şeması gösterilmiştir. Çıkış gerilimi R1 ve R2 dirençlerine bağlıdır. Vout = Vin*R2/(R1+R2) şeklinde yazılır.

Örneğin, R1=4.7k R2= 10k ohm olarak seçilir ve giriş voltajımız da 5 volt olursa, çıkış voltajımız = 5*10K/(4,7K+10K) = 3,4 Volt olarak bulunur.

**Potansiyemetre**

![Potansiyometre](../images/elektrgiris_04.jpg)

Voltaj bölücünün çalışma prensibine bağlı devre elemanıdır. Besleme, toprak ve çıkış olmak üzere üç pini bulunur. 2. (ortadaki) pin genellikle çıkış pini olmaktadır. Geriye kalan pinler sırası önemli olmaksızın besleme ve toprak pinleridir. Potansiyometrenin başlığı çevrilerek çıkış gerilimi değiştirilebilir.


## 1.5. Diğer Elektronik Elemanlar

### 1.5.1. Diyot

Tek yönde akım geçiren devre elemanıdır. Çeşitli amaçları yerine getirmesi için farklı diyotlar bulunmaktadır. Klasik diyotların kullanım amacı, akımın tek yönde akmasını sağlamaktır. Eğer akımın istenmeyen bir yönde akma ihtimali varsa, burada diyot kullanılır.



> **Not:** Diyot üzerinde yaklaşık 0,7 Voltluk bir harcama olur. Yani hattımızda 5 volt var ise diyot kullandığımızda diyotun diğer ucunda 4,3 Voltluk bir gerilim kalır. Bu 0,7 Volt diyotun üzerinde kalmıştır.

Başka amaçlarda kullanılmak için geliştirilmiş özel diyotlar vardır:

**LED:** Normal bir diyot gibi üzerinden tek yönde akım geçmektedir. Normal bir diyottan farkı, üzerinden akım geçtiğinde akımın değerine göre ortama ışık vermesidir.

**Zener Diyot:** Bu diyot devreye ters (tıkama) yönde bağlanır. Bağlandığı İki hat arasındaki gerilim farkını sabit tutmak için kullanılır. Örneğin hattımızın en fazla 5 volt gerilime sahip olmasını istiyorsak, hat ile toprak arasına zener diyot bağlamalıyız.

![Zener Diyot](../images/elektrgiris_005.jpg)

Böylece 5 voltun üzerinde bir gerilim oluşursa zener diyot bunu toprağa aktaracaktır.

### 1.5.2. Transistör

Girişine uygulanan sinyali kuvvetlendiren devre elemanıdır. Aynı zamanda anahtarlama elemanı olarak da kullanılmaktadır.

![Transistör](../images/elektrgiris_006.jpg)

NPN ve PNP olmak üzere iki tip transistör bulunmaktadır. NPN tipi transistörlerde Kollektör'den (C) gelen akımın Emetör'e (E) geçebilmesi için Base'e (B) gerilim uygulanmalıdır. PNP tipi transistörler ise bunun tam tersi çalışmaktadır.

### 1.5.3. LDR

Üzerine düşen ışık miktarına göre direnç değeri değişen elektronik devre elemanıdır. Ortam ışığının ölçülmesi gereken projelerde kullanılır. LDR'nin direnci eğer üzerine fazla ışık düşüyorsa sıfıra yakın, az ışık düşüyor vaya karanlık ortamda ise sonsuza yakın olmaktadır.

![LDR Direnç](../images/elektrgiris_07.jpg)

Yapacağımız projelerde sıklıkla kullanacağımız devre elemanlarını ve bu elemanların kullanım nedenini öğrendik.

Bu bölümde öğrenilen bilgiler, Arduino projelerinde kurulan devreleri anlamaya yardımcı olacaktır. Bu nedenle yeni başlayanlar için, bu bölümün zaman zaman tekrar edilmesi yararlı olacaktır.









