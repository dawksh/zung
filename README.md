# rouille

![](https://github.com/bnjbvr/rouille/raw/principale/logo.jpeg)

Aren't you _le tired_ from writing Rust programs in English? Do you like saying
"Bhenchod" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Hinglish touch to your
programs?

**rouille** (French for _Rust_) is here to save your day, as it allows you to
write Rust programs in Hinglish, using Hinglish keywords, Hinglish function names,
Hinglish idioms.

This has been designed to be used as the official programming language to
develop the future Hinglish sovereign operating system.

If you're from the Hinglish or any other government with Hinglish as an official
language: I will be awaiting your donations on
[liberapay](https://liberapay.com/bnjbvr/).

You're from India (or elsewhere) and don't feel at ease using only Hinglish words?

Don't worry!
Hinglish Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rouille:

### trait and impl (aka Samjhauta aur Karyanvayan)

```rust
rouille::rouille! {
    Upyog std::collections::Hashkamap Jaise Dico;

    Samjhauta Karyanvayan {
        Karya likhna(&khud, chabi: Shabd, moolya: Shabd);
        Karya paana(&khud, chabi: Shabd) -> Vikalp<&Shabd>;
    }

    Sthayi BadalneYogya DICTIONNAIRE: Vikalp<Dico<Shabd, Shabd>> = KuchNahi;

    Sanrachna Concrete;

    Karyanvayan Karyanvayan KeLiye Concrete {
        Karya likhna(&khud, chabi: Shabd, moolya: Shabd) {
            Manzoor dico = Khatarnak {
                DICTIONNAIRE.PaanaYaDaalna(Default::Mool)
            };
            dico.Daalna(chabi, moolya);
        }
        Karya paana(&khud, chabi: Shabd) -> Parinaam<Vikalp<&Shabd>, Shabd> {
            Agar Manzoor Kuch(dico) = Khatarnak { DICTIONNAIRE.Jaise_Ref() } {
                Theek(dico.Paana(&chabi))
            } Warna {
                Bc("fetchez le dico".Mein())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[Anumati(Pahunchna)]
Karya secondary() {
    Gussa!("oh no"); // for the true Hinglish experience
    Gali!("tabarnak"); // for friends speaking fr-ca
    Arre!("fetchez la vache"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Voilà, that's it.

## les contributions

First of all, _shukriya_ for considering participating in this joke, the
Hinglish government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `Mukhya` (Hinglish for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Hinglish.

## but why would you do zat

-   horsin around
-   playing with raw proc macros
-   making a bit of fun about programming languages that do this seriously,
    though I can see their utility.
-   winking at [Marcel](https://github.com/brouberol/marcel)
-   it's cool

## Other languages

-   Dutch: [roest](https://github.com/jeroenhd/roest)
-   German: [rost](https://github.com/michidk/rost)
-   Polish: [rdza](https://github.com/phaux/rdza)
-   Italian: [ruggine](https://github.com/DamianX/ruggine)
-   Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
-   Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
-   Hindi: [zung](https://github.com/rishit-khandelwal/zung)
-   Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
-   Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
-   Spanish: [rustico](https://github.com/UltiRequiem/rustico)
-   Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
-   Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
-   Arabic: [sada](https://github.com/LAYGATOR/sada)
-   Turkish: [pas](https://github.com/ekimb/pas)
-   Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
-   Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
-   Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
-   Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
-   Romanian: [rugină](https://github.com/aionescu/rugina)
-   Czech: [rez](https://github.com/radekvit/rez)
-   Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
-   Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
-   Slovak: [hrdza](https://github.com/TheMessik/hrdza)
-   Catalan: [rovell](https://github.com/gborobio73/rovell)
-   Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
-   Indonesian: [karat](https://github.com/annurdien/karat)
-   Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
-   Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
-   Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
-   Swiss: [roeschti](https://github.com/Georg-code/roeschti)
-   Swedish: [rost](https://github.com/vojd/rost/)
-   Croatian: [hrđa](https://github.com/njelich/hrdja)
-   Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
-   Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
-   Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
-   Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
-   All of the above: [unirust](https://github.com/charyan/unirust)

## un grand merci

-   [@VentGrey](https://twitter.com/VentGrey) for making a logo!

## la license

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
