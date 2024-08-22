
## Usage

  **Simple info (will scan from current dir and list the top 5 most recently accessed files)**
   ```bash
  lacs
  ```
 
 **Display the 10 most recently accessed files:**
  ```bash
  lacs -n 10
  ```
**Display files matching a specific pattern:**
```bash
lacs -g "*.txt,*.md"
```
**Display the 5 oldest accessed files:**
```bash
lacs -n 5 -o
```
