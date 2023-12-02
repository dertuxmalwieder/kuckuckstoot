# Die Kuckucksuhr (im Fediverse)

[Kuckuck!](https://social.tchncs.de/@kuckucksuhr)

## Kompilieren

    % cargo build --release

## Konfigurieren

    % mv kuckuck.toml-dist kuckuck.toml
    % ed kuckuck.toml

(Ihr braucht einen Mastodonaccount. Die Token kriegt ihr in den Entwicklereinstellungen in eurem Profil.)

## Laufen lassen

    % crontab -e
    # ...
    0 * * * * /home/kuckucksuhr/kuckucksuhr

## Lizenz

CDDL 1.1.
