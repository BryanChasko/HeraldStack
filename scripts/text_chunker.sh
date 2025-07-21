#!/bin/bash
# text_chunker.sh - Advanced text chunking utility for optimal embedding generation
#
# This utility provides multiple strategies for chunking text:
# 1. Size-based: Simply splits text at character count boundaries
# 2. Character-based: Splits at word boundaries to preserve semantic units
# 3. Semantic: Splits at natural breaks like sentences and paragraphs
#
# Usage:
#   ./text_chunker.sh --size <max-size> "Your text to chunk"
#   ./text_chunker.sh --char <target-size> "Your text to chunk"
#   ./text_chunker.sh --semantic "Your text to chunk with. Multiple sentences. Will be split."
#   ./text_chunker.sh --size <max-size> --file input.txt
#   cat input.txt | ./text_chunker.sh --char <target-size>
#
# Output formats:
#   --json         Output chunks as JSON array
#   --numbered     Output chunks with line numbers
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -e

# Default settings
MAX_SIZE=250
MODE="size"
INPUT=""
INPUT_FILE=""
OUTPUT_FORMAT="plain"
PRESERVE_WHITESPACE=false
DELIMITER=""
DEBUG=false

# Functions for chunking strategies
size_based_chunking() {
    local text="$1"
    local max_size="$2"
    local chunks=()
    local length=${#text}
    
    if [ $length -le $max_size ]; then
        chunks+=("$text")
    else
        local i=0
        while [ $i -lt $length ]; do
            chunks+=("${text:$i:$max_size}")
            i=$((i + max_size))
        done
    fi
    
    printf "%s\n" "${chunks[@]}"
}

character_based_chunking() {
    local text="$1"
    local target_size="$2"
    local chunks=()
    local length=${#text}
    
    if [ $length -le $target_size ]; then
        chunks+=("$text")
    else
        local start=0
        while [ $start -lt $length ]; do
            # If we're near the end of the text
            if [ $((start + target_size)) -ge $length ]; then
                chunks+=("${text:$start}")
                break
            fi
            
            # Find a good breakpoint near the target size
            local end=$((start + target_size))
            
            # Look backward from target point to find a word boundary
            while [ $end -gt $start ] && [[ "${text:$end:1}" != " " && "${text:$end:1}" != $'\n' && 
                                           "${text:$end:1}" != "." && "${text:$end:1}" != "," && 
                                           "${text:$end:1}" != "!" && "${text:$end:1}" != "?" ]]; do
                end=$((end - 1))
            done
            
            # If no good breakpoint found, just break at target size
            if [ $end -eq $start ]; then
                end=$((start + target_size))
            else
                # Include the delimiter in this chunk
                end=$((end + 1))
            fi
            
            chunks+=("${text:$start:$((end - start))}")
            start=$end
            
            # Skip any extra spaces at the beginning of the next chunk
            while [ $start -lt $length ] && [[ "${text:$start:1}" == " " || "${text:$start:1}" == $'\n' ]]; do
                start=$((start + 1))
            done
        done
    fi
    
    printf "%s\n" "${chunks[@]}"
}

semantic_chunking() {
    local text="$1"
    local delimiter="$2"
    local chunks=()
    
    # Default delimiters if none provided
    if [ -z "$delimiter" ]; then
        # Split on sentence boundaries and paragraph breaks
        IFS=$'\n' read -d '' -ra paragraphs < <(echo -e "$text" && printf '\0')
        
        for paragraph in "${paragraphs[@]}"; do
            if [ -z "$paragraph" ]; then
                continue
            fi
            
            if [[ "$paragraph" =~ ^#+ || "$paragraph" =~ ^- ]]; then
                # Treat headings and list items as single chunks
                chunks+=("$paragraph")
            else
                # Split paragraph into sentences
                local sentences=$(echo "$paragraph" | sed 's/\([.!?]\) /\1\n/g')
                IFS=$'\n' read -d '' -ra sentence_array < <(echo -e "$sentences" && printf '\0')
                
                for sentence in "${sentence_array[@]}"; do
                    if [ -n "$sentence" ]; then
                        chunks+=("$sentence")
                    fi
                done
            fi
        done
    else
        # Split on custom delimiter
        IFS="$delimiter" read -d '' -ra chunks < <(echo -e "$text" && printf '\0')
    fi
    
    printf "%s\n" "${chunks[@]}"
}

format_output() {
    local format="$1"
    shift
    local chunks=("$@")
    
    if [ "$format" == "json" ]; then
        local json_array="["
        local first=true
        for chunk in "${chunks[@]}"; do
            if [ "$first" == true ]; then
                first=false
            else
                json_array+=","
            fi
            # Escape special characters for JSON
            local escaped_chunk=$(echo "$chunk" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | sed 's/\n/\\n/g')
            json_array+="\"$escaped_chunk\""
        done
        json_array+="]"
        echo "$json_array"
    elif [ "$format" == "numbered" ]; then
        local i=1
        for chunk in "${chunks[@]}"; do
            echo "$i: $chunk"
            i=$((i + 1))
        done
    else
        for chunk in "${chunks[@]}"; do
            echo "$chunk"
        done
    fi
}

process_input() {
    local text="$1"
    local mode="$2"
    local size="$3"
    local delimiter="$4"
    local chunks=()
    
    # Remove excess whitespace unless preserving
    if [ "$PRESERVE_WHITESPACE" == false ]; then
        if [ "$DEBUG" == true ]; then
            echo "Removing excess whitespace" >&2
        fi
        text=$(echo "$text" | tr -s ' ' | sed 's/^ //g' | sed 's/ $//g')
    fi
    
    # Create a temporary file to store chunks
    local temp_file=$(mktemp)
    
    if [ "$mode" == "size" ]; then
        if [ "$DEBUG" == true ]; then
            echo "Using size-based chunking (max: $size chars)" >&2
        fi
        size_based_chunking "$text" "$size" > "$temp_file"
    elif [ "$mode" == "char" ]; then
        if [ "$DEBUG" == true ]; then
            echo "Using character-based chunking (target: $size chars)" >&2
        fi
        character_based_chunking "$text" "$size" > "$temp_file"
    elif [ "$mode" == "semantic" ]; then
        if [ "$DEBUG" == true ]; then
            echo "Using semantic chunking" >&2
            [ -n "$delimiter" ] && echo "With custom delimiter: '$delimiter'" >&2
        fi
        semantic_chunking "$text" "$delimiter" > "$temp_file"
    else
        echo "Error: Unknown chunking mode: $mode" >&2
        rm -f "$temp_file"
        exit 1
    fi
    
    # Read chunks from temp file
    chunks=()
    while IFS= read -r line; do
        chunks+=("$line")
    done < "$temp_file"
    rm -f "$temp_file"
    
    format_output "$OUTPUT_FORMAT" "${chunks[@]}"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --size)
            MODE="size"
            MAX_SIZE="$2"
            shift 2
            ;;
        --char)
            MODE="char"
            MAX_SIZE="$2"
            shift 2
            ;;
        --semantic)
            MODE="semantic"
            shift
            ;;
        --file|-f)
            INPUT_FILE="$2"
            shift 2
            ;;
        --json)
            OUTPUT_FORMAT="json"
            shift
            ;;
        --numbered)
            OUTPUT_FORMAT="numbered"
            shift
            ;;
        --preserve-whitespace)
            PRESERVE_WHITESPACE=true
            shift
            ;;
        --delimiter)
            DELIMITER="$2"
            shift 2
            ;;
        --debug)
            DEBUG=true
            shift
            ;;
        --help|-h)
            echo "Usage:"
            echo "  ./text_chunker.sh --size <max-size> \"Your text to chunk\""
            echo "  ./text_chunker.sh --char <target-size> \"Your text to chunk\""
            echo "  ./text_chunker.sh --semantic \"Your text with. Multiple sentences.\""
            echo "  ./text_chunker.sh --size <max-size> --file input.txt"
            echo "  cat input.txt | ./text_chunker.sh --char <target-size>"
            echo ""
            echo "Options:"
            echo "  --size <n>          Size-based chunking with maximum n characters"
            echo "  --char <n>          Character-based chunking targeting n characters per chunk"
            echo "  --semantic          Split at natural sentence and paragraph boundaries"
            echo "  --file, -f <file>   Read input from file"
            echo "  --json              Output as JSON array"
            echo "  --numbered          Output with line numbers"
            echo "  --preserve-whitespace  Don't normalize whitespace"
            echo "  --delimiter <delim>  Custom delimiter for semantic chunking"
            echo "  --debug             Show debug information"
            echo "  --help, -h          Show this help message"
            exit 0
            ;;
        *)
            if [ -z "$INPUT" ]; then
                INPUT="$1"
                shift
            else
                echo "Error: Unexpected argument: $1" >&2
                exit 1
            fi
            ;;
    esac
done

# Get input from argument, file, or stdin
if [ -n "$INPUT_FILE" ]; then
    if [ ! -f "$INPUT_FILE" ]; then
        echo "Error: Input file not found: $INPUT_FILE" >&2
        exit 1
    fi
    INPUT=$(cat "$INPUT_FILE")
elif [ -z "$INPUT" ]; then
    # Check if stdin is a terminal
    if [ -t 0 ]; then
        echo "Error: No input provided. Use --help for usage information." >&2
        exit 1
    else
        # Read from stdin
        INPUT=$(cat)
    fi
fi

# Process the input
process_input "$INPUT" "$MODE" "$MAX_SIZE" "$DELIMITER"
