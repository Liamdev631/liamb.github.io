# Visual Styling Guidelines

## Iconography
We use **vscode-icons** to mimic the authentic VS Code file explorer appearance.
Source: [vscode-icons GitHub Repository](https://github.com/vscode-icons/vscode-icons/tree/master/icons)

### Base URL
`https://raw.githubusercontent.com/vscode-icons/vscode-icons/master/icons/`

### Icon Mapping
| File Type | Icon Name | URL |
|-----------|-----------|-----|
| **Rust** (*.rs) | `file_type_rust.svg` | `.../file_type_rust.svg` |
| **Markdown** (*.md) | `file_type_markdown.svg` | `.../file_type_markdown.svg` |
| **C++** (*.cpp) | `file_type_cpp.svg` | `.../file_type_cpp.svg` |
| **HTML** (*.html) | `file_type_html.svg` | `.../file_type_html.svg` |
| **Project** (*.proj) | `file_type_vcxproj.svg` | `.../file_type_vcxproj.svg` |
| **Folder** (Closed) | `default_folder.svg` | `.../default_folder.svg` |
| **Folder** (Open) | `default_folder_opened.svg` | `.../default_folder_opened.svg` |
| **Unknown/Text** | `default_file.svg` | `.../default_file.svg` |

### Usage
Use these icons as `<img>` tags with a standard width/height (e.g., 14px or 16px) and `object-fit: contain`.
Do NOT apply CSS coloring (like `color: #dcb67a`) to these SVGs as they are pre-colored.

## Theme Colors
Refer to `index.html` root variables for the color palette, which is based on VS Code's "Dark Modern" theme.

## Fonts
- **Code**: `Consolas`, `Courier New`, monospace (default)
- **About Page**: `'Segoe UI'`, `Tahoma`, `Geneva`, `Verdana`, `sans-serif`
