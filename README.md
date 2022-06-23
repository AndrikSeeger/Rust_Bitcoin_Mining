# Bitcoin Mining Implementierung mittels Rust

Einfache Implementierung zum CPU-Minen von Bitcoins über einen Bitcoin Mining Pool.

Über das Stratum V2 Protokoll wird eine TCP-Verbindung mit dem Mining-Pool [SlushPool](https://slushpool.com/en/home/) aufgebaut.
Im Anschluss werden Daten vom Mining Pool geladen und in einen Job konvertiert.
Über eine Veränderung der Nonce wird nach möglichen Lösung gesucht.
Falls eine valide Lösung gefunden wird, wird diese über das Stratum Protokoll zurück an den Server geschickt.

# Zusätzliche Informationen

Hierbei handelt es sich lediglich um ein Proof-of-Concept ohne jegliche Beachtung der Berechnungseffizienz.
Mögliche Rendite und Erfolge beim Minen sind zwar nicht ausgeschlossen aber durch die einfache Implementierung und limitierte Rechenleistung von CPUs
sehr gering.

Der verwendete Slush-Pool Account kann in der Datei `main.rs` verändert werden.

