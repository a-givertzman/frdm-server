#
# Запускай так
#
# Полная очистка: убрать тесты и C-биндинги
# python bundler.py src/main.rs --no-tests --no-c
#
# Просто собрать в один файл (без исключений)
# python bundler.py src/main.rs
#
# Пропуск папок: --exclude-dirs
# Аргумент --exclude-dirs: Можно передать несколько папок через пробел.
# Абсолютные пути: Скрипт переводит относительные пути в абсолютные
# python bundler.py src/main.rs --exclude-dirs src/secret_stuff src/temp_tests --no-tests

import os
import re
import sys
import argparse
from pathlib import Path

def bundle_rust(file_path, exclude_dirs=None):
    """Рекурсивно собирает Rust файлы, пропуская указанные папки."""
    if exclude_dirs is None:
        exclude_dirs = []
    
    # Превращаем в абсолютные пути для надежного сравнения
    abs_exclude_dirs = [os.path.abspath(d) for d in exclude_dirs]
    abs_file_path = os.path.abspath(file_path)

    # Проверяем, не находится ли текущий файл в исключенном каталоге
    for ex_dir in abs_exclude_dirs:
        if abs_file_path.startswith(ex_dir):
            return f"// Skipped: {file_path} (excluded directory)\n"

    if not os.path.exists(file_path):
        return f"// Error: {file_path} not found"

    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    result = []
    base_dir = os.path.dirname(file_path)

    for line in lines:
        match = re.match(r'^(?P<prefix>.*?\bmod\s+)(?P<name>\w+);', line)
        if match:
            mod_name = match.group('name')
            prefix = match.group('prefix')
            
            # Поиск пути модуля
            mod_path = os.path.join(base_dir, f"{mod_name}.rs")
            if not os.path.exists(mod_path):
                mod_path = os.path.join(base_dir, mod_name, "mod.rs")

            if os.path.exists(mod_path):
                # Проверка исключений перед рекурсией
                is_excluded = False
                abs_mod_path = os.path.abspath(mod_path)
                for ex_dir in abs_exclude_dirs:
                    if abs_mod_path.startswith(ex_dir):
                        is_excluded = True
                        break
                
                if is_excluded:
                    result.append(f"// {prefix}{mod_name}; (Excluded)\n")
                else:
                    result.append(f"{prefix}{mod_name} {{\n")
                    result.append(bundle_rust(mod_path, exclude_dirs))
                    result.append(f"}}\n")
            else:
                result.append(line)
        else:
            result.append(line)

    return "".join(result)

def remove_comments_and_empty_lines(code):
    code = re.sub(r'/\*.*?\*/', '', code, flags=re.DOTALL)
    code = re.sub(r'//.*', '', code)
    lines = [line.rstrip() for line in code.splitlines() if line.strip()]
    return "\n".join(lines)

def find_block_end(code, start_idx):
    depth = 0
    in_block = False
    for i in range(start_idx, len(code)):
        char = code[i]
        if char == '{':
            depth += 1
            in_block = True
        elif char == '}':
            depth -= 1
            if in_block and depth == 0:
                return i + 1
        elif char == ';' and not in_block:
            return i + 1
    return len(code)

def strip_blocks(code, pattern_str):
    pattern = re.compile(pattern_str, re.MULTILINE)
    while True:
        match = pattern.search(code)
        if not match:
            break
        start = match.start()
        end = find_block_end(code, match.end())
        code = code[:start] + code[end:]
    return code

def main():
    parser = argparse.ArgumentParser(description="Rust Bundler with Directory Exclusions")
    parser.add_argument("input", help="Path to main.rs")
    parser.add_argument("-o", "--output", default="bundle.rs", help="Output file")
    parser.add_argument("--no-tests", action="store_true", help="Exclude #[test] blocks")
    parser.add_argument("--no-c", action="store_true", help="Exclude extern \"C\" blocks")
    parser.add_argument("--exclude-dirs", nargs='+', default=[], 
                        help="List of relative directories to skip (e.g., src/bin src/experimental)")
    
    args = parser.parse_args()

    print(f"🔄 Bundling {args.input}...")
    # Собираем код с учетом исключенных папок
    full_code = bundle_rust(args.input, args.exclude_dirs)
    
    print("🧹 Cleaning comments...")
    full_code = remove_comments_and_empty_lines(full_code)

    if args.no_tests:
        print("🚫 Stripping tests...")
        full_code = strip_blocks(full_code, r'#\[test\]')
        full_code = strip_blocks(full_code, r'#\[cfg\(test\)\]')

    if args.no_c:
        print("🚫 Stripping C-bindings...")
        full_code = strip_blocks(full_code, r'extern\s+"C"')

    with open(args.output, 'w', encoding='utf-8') as f:
        f.write(full_code)
    
    print(f"✅ Done! Saved to {args.output}")

if __name__ == "__main__":
    main()

#
# Запускай так
#
# Полная очистка: убрать тесты и C-биндинги
# python bundler.py src/main.rs --no-tests --no-c
#
# Просто собрать в один файл (без исключений)
# python bundler.py src/main.rs
#
# Пропуск папок: --exclude-dirs
# Аргумент --exclude-dirs: Можно передать несколько папок через пробел.
# Абсолютные пути: Скрипт переводит относительные пути в абсолютные
# python bundler.py src/main.rs --exclude-dirs src/secret_stuff src/temp_tests --no-tests
