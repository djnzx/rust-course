convert `coursework-checklist/coursework-checklist.md` to `coursework-checklist/coursework-checklist.docx` (pandoc, then patch the XML):

- page: A4 landscape (297×210 mm)
- margins: narrow, 1.27 cm on all four sides (top = left)
- body: 3 columns, 1.27 cm gap
- font: Times New Roman in all styles (headings included)
- title (the `#` heading):
  - one-column section, so it spans all 3 columns; the body starts on the same page right below it (continuous section break)
  - centred, black text, 10% grey background (fill `E6E6E6`)
  - no space before it (the text starts at the top margin), 12 pt space after it
  - the paragraph that carries the section break has a 1 pt line height, so no blank line shows
- overwrite the existing docx; check that it opens (`pandoc ... -t plain`)
