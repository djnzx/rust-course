convert `coursework-checklist/coursework-checklist.docx` to `coursework-checklist/coursework-checklist.pdf`, keeping the Word fonts (Times New Roman).

Why a font step is needed: headless LibreOffice does not see the macOS system fonts and silently replaces Times New Roman with Liberation Serif. Copy the font into LibreOffice's user font folder once (it stays on the machine):

```sh
D=~/Library/Application\ Support/LibreOffice/4/user/fonts
mkdir -p "$D"
cp /System/Library/Fonts/Supplemental/Times\ New\ Roman*.ttf "$D"/
```

Convert:

```sh
cd coursework-checklist
soffice --headless --convert-to pdf coursework-checklist.docx
```

Check:

```sh
pdffonts coursework-checklist.pdf   # TimesNewRomanPSMT, not LiberationSerif
pdfinfo coursework-checklist.pdf    # A4 landscape: 841.89 x 595.304 pts, 1 page
```

- overwrite the existing pdf
- render a page (`pdftoppm -png -r 80`) and look at it: grey title bar over 3 columns, nothing cut off
