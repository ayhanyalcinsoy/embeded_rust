![Rust Logo](../images/rust_logo.png)__________________________________________![Arduino Logo](../images/arduino_logo.png)

## Neden Rust?

Gömülü sistemler teknolojisi onlarca yıldır yenilikten yoksundu. Yıldırım hızında, gömülü cihazları programlamak için tercih edilen dil uzun zamandır C/C++ olmuştur, ancak Rust daha da hızlı geliştirme desteği sağlar. Rust gömülü sistem geliştirme için mükemmel bir seçimdir çünkü: 

* C kod tabanlarıyla yüksek oranda birlikte çalışabilir 
* Taşınabilir ve hafiftir 
* Güçlü bir eşzamanlılık modelidir 
* Farklı mikrodenetleyiciler için sağlam destek sunar 
* Bellek güvenlidir 

Arduino'ları zaten C++ ile programladıysanız, temelleri öğrendikten sonra bunu Rust ile yapmaya geçmek nispeten kolay olacaktır. Gömülü Rust hakkında daha fazla bilgiyi buradan edinebilirsiniz. Ayrıca mevcut mikrodenetleyici kasalarını da buradan görebilirsiniz.

***Arduino ile gömülü sistem geliştirme için normal prosedür aşağıdaki adımları içerir:***

 * Amaçlanan devrenin elektrik şemasının çizilmesi 
 * Elektrik bileşenlerinin şemaya uyacak şekilde bağlanması 
 * Devreyi istenildiği gibi kontrol etmek için program mantığının yazılması 
 * Mikrodenetleyicinin USB kablosuyla bilgisayara bağlanması 
 * Programın bilgisayardan kartın flash belleğine aktarılması(veya yüklenmesi) 

### Gerekli Araçlar:
Burada anlatılanları yapabilmek için bir Arduino kartına ve aşağıdaki yazılım önkoşullarına ihtiyacınız olacak:

 * Program yazma, derleme ve yazılan programı arduino karta aktarmak için bir bilgisayar
 * Cargo yazılımı
 * Rust gecelik derleyici sürümü

## Kurulum ve Ayarlar

### avrdude kullanmak
 
avrdude, avr-hal projeleri için kargo tarafından oluşturulan bir şablondur. Şu anda aşağıdaki donanımları desteklemektedir:

 * Arduino Leonardo
 * Arduino Mega 2560
 * Arduino Mega 1280
 * Arduino Nano
 * Arduino Nano New Bootloader (Ocak 2018'den sonra üretildi)
 * Arduino Uno
 * SparkFun ProMicro
 * SpartFun ProMini 3.3V
 * SpartFun ProMini 5v
 * Adafruit Trinket
 * Adafruit Trinket Pro

AVR mikrodenetleyicileri ve diğer yaygın kartlarda Rust çalıştırmak için bir Donanım Soyutlama Katmanı (HAL) gereklidir. Bunu elde etmek için, makinenizde Rust kodunu AVR'ye derleyen gecelik Rust derleyicisine ihtiyacınız vardır. 

### Pardus

Pardus gibi bir Linux dağıtımı kullanıyorsanız aşağıdaki komut ile gerekli paketler yüklenir: 

`sudo apt install avr-libc gcc-avr pkg-config avrdude libssl-dev libudev-dev build-essential `

Aşağıdaki komut ile rustup araç zinciri olmadan (toolchain) sisteme kurulur.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y`

Daha sonra gecelik yayımlanan araç zinciri (toolchain) aşağıdaki komut ile sisteme kurulur.

`rustup toolchain install nightly --allow-downgrade --profile minimal --component clippy`


### Bash, Fish, Zsh, or PowerShell için Tab (sekme) tamamlamayı etkinleştirme

rustup artık Bash, Fish, Zsh ve PowerShell için tamamlama betikleri oluşturmayı destekliyor. Tüm ayrıntılar için rustup yardım tamamlamalarına bakın, ancak özü aşağıdakilerden birini kullanmak kadar basittir:

***Bash***
`$ rustup completions bash > ~/.local/share/bash-completion/completions/rustup`

*** Bash (macOS/Homebrew)***
`$ rustup completions bash > $(brew --prefix)/etc/bash_completion.d/rustup.bash-completion`

***Fish***
`$ mkdir -p ~/.config/fish/completions`
`$ rustup completions fish > ~/.config/fish/completions/rustup.fish`

***Zsh***
`$ rustup completions zsh > ~/.zfunc/_rustup`

***PowerShell v5.0+***
`$ rustup completions powershell >> $PROFILE.CurrentUserCurrentHost`
*** ya da***
`$ rustup completions powershell | Out-String | Invoke-Expression`


Yukarıdaki kılavuzu takip ederken bir engelle karşılaşırsanız, burada tüm işletim sistemleri için bir kurulum kılavuzu vardır. Bu adımlardan herhangi birinden sonra, bir sonraki adım, mikrodenetleyici kartını kargoya karşı bağlantılamak için ravedude aracını yüklemektir: 

`cargo install ravedude`

Bu araç, kartı bulmakta, yazılan kodu karta aktarma  ve bağlantıları dinleme işlerini yerine getirir. Tek yapmanız gereken **cargo run** komutunu çalıştırmak.


## Avrdude ile yeni bir Arduino projesi oluşturma

Yeni bir proje başlatmak cargo-generate sandığı ile daha basit hale getirilmiştir. Yeni bir proje oluşturmak için aşağıdaki komutları art arda çalıştırmanız yeterlidir:

`cargo install cargo-generate`

Şimdi, şablonu oluşturmak ve örneklemek için bu komutu çalıştırın. Şu anda bir proje oluşturmadınız, ancak araç bunu halledecektir:

`cargo generate --git https://github.com/Rahix/avr-hal-template.git`

Komutu çalıştırdıktan sonra, projeniz için bir ad belirtmek üzere bir giriş alanı görmelisiniz. Bu eğitimde proje adı olarak **"rust-x-arduino"** kullanılacaktır. 

Tercih ettiğiniz adı girdikten sonra Enter tuşuna tıklayın. Bir sonraki günlük, avrdude şablonu altında bulunan mikrodenetleyicilerin bir listesini gösterir. Bu makale, herkesin kolayca kullanabileceği bir varyant olan Arduino UNO'yu kullanmaktadır. 

Derlemeden sonra projeye gidin ve klasörü tercih ettiğiniz kod düzenleyicide bir proje olarak açın. Proje yapısı aşağıdaki resimdeki gibi görünmelidir:

![Proje Yapısı](../images/project-structure.png)

Not: libudev-sys crate'i yüklerken bir hata oluşursa, bunu bağımlılıklar altındaki cargo.toml dosyanıza eklemeniz gerekecektir:

`[dependencies]`

`libudev-sys = "0.1"`

**libudev** Rust binding, libudev C kütüphanesi için bildirimler ve bağlantı sağlayan bir sandıktır. Linux'a özgüdür, bu nedenle Windows veya OSX işletim sistemleri için mevcut değildir. Alternatif olarak, libudev-sys crate'ini yüklemek için aşağıdaki komutu çalıştırabilirsiniz:

`sudo apt-get install libudev-dev`

**pkg-config**'den kaynaklanan başka sorunlar olması durumunda libudev-sys deposuna başvurun. Şimdi, build komutu ile projeyi derleyebilirsiniz:

`cargo build`

Bu işlem CPU yoğun bir görev olduğu için biraz zaman alabilir. Daha sonra, `target/avr-atmega328p/debug/` altında bir .elf dosyası bulacaksınız. Aynı zamanda bir de .hex dosyası bulacaksınız. Hex uzantılı dosya simulIDE ile projemizi çalıştırmak için kullanacağımız dosyadır. Eğer .hex uzantılı dosya oluşmaz ise aşağıdaki komutu kullanarak .elf dosyasından bir .hex dosyası elde edebilirsiniz.

`avr-objcopy -O ihex target/avr-atmega328p/debug/rust-x-arduino.elf target/avr-atmega328p/debug/rust-x-arduino.hex`

Kendi programınızı çalıştırmak için, temel bir LED Yanıp Sönme programı için örnek bir kod içeren main.rs dosyasını aşağıdaki gibi düzenleyebilirsiniz:

![Rust kodu](../images/blink.png)

### Gömülü Rust Kodunu Anlamak
Kodun ilk iki satırından, işletim sistemi olmayan gömülü bir proje olduğu için standart bir kütüphane ve main olmadığı açıktır.
 
`#[arduino_hal::entry]` satırı programdaki giriş noktasını belirtir.  `panic_halt as_;` panikleri işlemek için kullanılır. 

**main** fonksiyonunda, Çevre Birimleri çözülür. Gömülü Rust'ta Çevre Birimleri, çevrelerini anlamlandıran ve insanlarla etkileşime giren bileşenleri ifade eder. Sensörler, aktüatörler ve motor kontrolörlerinin yanı sıra CPU, RAM veya flash bellek gibi mikrodenetleyicinin temel parçalarını da içerirler. Gömülü Rust kitabında Çevre Birimleri hakkında daha fazla bilgi edinebilirsiniz. 

Ardından, varsayılan pinin (_D13_) dijital çıkışını yükseğe ayarlamak için Arduino kartının pinlerine erişim sağlıyoruz. 

Döngüdeki **toggle** yöntemi LED'i açıp kapatmak için kullanılırken, **delay_ms** yöntemi döngüyü belirtilen milisaniye kadar geciktirmek için kullanılır.

## Kod yükleme için Mikrodenetleyiciyi yapılandırma
Resmi Arduino IDE'sinde Arduino mikrodenetleyicisi ile çalışırken, programı C++ tabanlı olan Arduino'da yazmanız ve program kaynak dosyasını USB portu üzerinden karta yüklemeniz yeterlidir. Rust ile daha uzun ama benzer bir prosedür izleyeceğiz. Linux komutu ile makinenizdeki açık USB portlarını listeleyerek başlayın: 

`lsusb`

Arduino kartınız USB üzerinden cihazınıza takılıysa, aşağıdaki görüntüdeki gibi Arduino kartına bağlı USB'nin adını görmelisiniz:

![USB Arduino Board](../images/usb-arduino-board.png)

Daha sonra, bu betik ile ravedude için seri com portunu ayarlayacağız:

`export RAVEDUDE_PORT=/dev/ttyUSB0`

Bu, ravedude'a Arduino'nun hangi porta bağlı olduğunu söyler. Aşağıdaki komutu çalıştırmak, programı Arduino'ya yükleyecektir:

`cargo run`

## Mikro denetleyici üzerindeki çıktı

Program mikrodenetleyiciye yüklendiğinde Arduino programlandığı gibi davranacaktır. Bu durumda kart üzerindeki LED ışıklar programda belirtilen zaman aralıklarına göre yanıp sönecektir:
![Blinking LED](../images/rust-arduino-uno-final-1.gif)

