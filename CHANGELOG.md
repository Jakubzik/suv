# version 0.0.7

- wenn's keine nächsten Schritte gibt, entsprechende Mitteilung
- manpage aktualisieren
- keine `panic` bei unbekanntem Befehl

# version 0.0.6

- Fehler "SuvError"

# version 0.0.5

- Umsortieren der Commands in vernünftigere Module
- `version`

# version 0.0.4

- direct command: suv list_next, list_all
- Tests

# version 0.0.3

- Ask: add new or view files
- Add new file to folder with basic info

# version 0.0.2

- Version number,
- Implement basic configuration (folder where data is to be stored)

# version 0.0.1

Created Github repo.

# Unreleased

[ ] Zweitgutachter eintragen können
[ ] make .edit_config suggest `$suv_folder/suv/archive` as default for an archive (in main.rs/get_suv_folder)
[ ] Refine add thesis in ui/mod.rs get_thesis_details_from_user
  [ ] thesis title
  [ ] Zweitgutachter
  [ ] How to do the dates (Abgabe -- auch vage Angaben wie "Januar 26"?)
  [ ] Mehr Defaults bei der Eingabe
[ ] Implement "vorhandene Daten einsehen"
  [ ] Collect all files (i.e. student names) in config-folder
  [ ] READ files in config-folder in ui/mod.rs impl Thesis -> .from_file
  [ ] Überlegen welche Interaktionen zweckmäßig wären
[ ] Decide about cli interface (dialoguer?)\

# History
[x] Bugfix: teilw. Großschreibung der Namen führt zu blöden Problemen beim Speichern einer neuen Thesis. [0.0.5]
