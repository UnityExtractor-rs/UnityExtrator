## UnityExtractor v0.1.1

## :sparkles: New Feature

1. support extract following unity asset type 
    - Texture2D
    - TextAsset
    - Sprite

2. support export extracted information

Click the prepend icon of each item in the list, 
then select `Export Full` or `Save Object`. 
the `UnityExtractor` will export all information to the target directory

3. Preview Image or Encodable String

Click any Item of a Unity Asset, if it is an Image or Utf8 string bytes, 
you can preview the value in a independence windows

## :bug: Bug Fix

- fix: unpredicted exit when loading non unity asset file
- fix: Exit main window not exit