# SraP

Simple readable application Programming-Language — kurz: SraP — ist ein kleines, experimentelles Projekt: eine eigens entwickelte Programmiersprache und ein dazugehöriger Compiler. Ziel dieses Repositories ist es, das Design und die Implementierung einer Sprache zu erkunden und dabei Rust besser kennenzulernen.

> Hinweis: Die Entwicklung ist noch sehr am Anfang. Vieles ist momentan konzeptionell oder als Prototyp vorhanden. Das Projekt dient primär als Lernprojekt und wird stetig aktualisiert.

## Motivation

- Lernen von Rust anhand eines handfesten Projekts.
- Verständnis für Sprachelemente (Lexer, Parser, AST, Type-Checker, Code-Generation).
- Aufbau eines einfachen Compilers und Erforschung von Designentscheidungen für eine "simple readable" Sprache.

## Aktueller Status

- Sprache: sehr frühe Phase (Proof-of-Concept / Prototyp).
- Implementierung: erste Parser-/Lexer-Ansätze, vereinzelte AST-Strukturen und Hilfswerkzeuge.
- Stabilität: experimentell — nicht produktiv verwenden.
- Updates: regelmäßige kleine Commits, Dokumentation wird schrittweise erweitert.

## Ziele (kurz-/mittelfristig)

- Minimaler Lexer + Parser für einfache Ausdrücke.
- Repräsentation als AST und einfache Auswertungslogik.
- Kleine Standardbibliothek mit Ein-/Ausgabe-Funktionen.
- Optional: einfache Typprüfung und Fehlermeldungen.
- Langfristig: mehr Sprachkonstrukte, bessere Tests, evtl. Code-Generation.

## Technologie

- Implementiert in Rust (100% Rust-Codebasis).
- Baut auf der Rust-Toolchain auf (cargo).

## Schnellstart

Voraussetzungen:
- Rust (empfohlen: aktuelle stabile Version)
- Git

Klonen und bauen:
```bash
git clone https://github.com/duck-overflow/SraP.git
cd SraP
cargo build
```

Tests (sofern vorhanden):
```bash
cargo test
```

Wie man das Projekt benutzt oder Beispiele ausführt wird fortlaufend ergänzt — aktuell gibt es noch keine stabilen CLI-Befehle oder Beispielprogramme. Schau dir den Quellcode an, um den aktuellen Stand der Implementierung zu sehen.

## Wie du beitragen kannst

Das Projekt freut sich über Beiträge, Fragen und Feedback — besonders, wenn du auch Rust lernen oder Erfahrungen zum Design von Programmiersprachen teilen möchtest.

- Issues eröffnen für Bugs, Designvorschläge oder Fragen.
- Kleine PRs für klare Verbesserungen (z. B. Tests, Dokumentation, Refactorings).
- Vor dem Arbeiten an größeren Änderungen gerne kurz ein Issue anlegen, damit wir das Design abstimmen können.

Konventionen:
- Schreibe klare Commit- und PR-Beschreibungen.
- Kurze, fokussierte Pull Requests sind willkommen.

## Roadmap / To‑Dos

- [ ] Stabilen Parser für grundlegende Ausdrücke implementieren
- [ ] Einfachen Compiler hinzufügen
- [ ] Erste Beispiele / Tutorials in docs/
- [ ] Continuous Integration (Tests)
- [ ] Dokumentation der Sprachsyntax

Diese Liste wird regelmäßig aktualisiert.

## Lizenz

Dieses Projekt ist lizenziert unter der MIT-Lizenz — siehe LICENSE-Datei für Details.

## Kontakt

Repository: https://github.com/duck-overflow/SraP

Wenn du Fragen oder Vorschläge hast, öffne gern ein Issue oder erstelle einen Pull Request.

---
Danke fürs Vorbeischauen — das Projekt wächst schrittweise. Schau regelmäßig vorbei oder abonnier das Repository, um Updates zu erhalten.
