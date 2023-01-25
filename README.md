# Die Kuckucksuhr

[Kuckuck!](https://social.tchncs.de/@kuckucksuhr)

(Dies ist die Mastodon-Version - die Kuckucksuhr fÃr Twitter findet ihr [hier.](https://code.rosaelefanten.org/kuckucksuhr)

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
