# Requirements:
## 1. Rust
## 2. yt-dlp

# How to use args.txt

## Structure of file
**first line** = Path(optional)<br>
**second line** = Output<br>
**third line** = Format<br>

## For audio(example):
```
out
audio
opus(value or "")
```

## For video(example):
```
out
video
399+251(value or "")
```

## You don't have to use this file, but it's more efficient then manualy type in cli. You always have a choise