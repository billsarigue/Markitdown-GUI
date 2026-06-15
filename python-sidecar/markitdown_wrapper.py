#!/usr/bin/env python3
"""
Markitdown wrapper for Tauri sidecar.
Usage: markitdown_wrapper <input_file> [output_file] [--enable-plugins] [--llm-endpoint URL] [--llm-deployment NAME]
"""

import sys
import os
import json
import argparse
from markitdown import MarkItDown


def main():
    parser = argparse.ArgumentParser(description="Convert files to Markdown via MarkItDown")
    parser.add_argument("input", help="Path to the input file")
    parser.add_argument("output", nargs="?", default=None, help="Path to the output .md file (optional)")
    parser.add_argument("--enable-plugins", action="store_true", help="Enable MarkItDown plugins")
    parser.add_argument("--llm-endpoint", default=None, help="Azure OpenAI endpoint for image analysis")
    parser.add_argument("--llm-deployment", default=None, help="Azure OpenAI deployment name")
    parser.add_argument("--llm-api-key", default=None, help="API key for LLM")

    args = parser.parse_args()

    if not os.path.exists(args.input):
        print(json.dumps({"success": False, "error": f"File not found: {args.input}"}))
        sys.exit(1)

    try:
        kwargs = {"enable_plugins": args.enable_plugins}

        if args.llm_endpoint and args.llm_deployment and args.llm_api_key:
            from openai import AzureOpenAI
            llm_client = AzureOpenAI(
                api_key=args.llm_api_key,
                azure_endpoint=args.llm_endpoint,
                api_version="2024-02-01",
            )
            kwargs["llm_client"] = llm_client
            kwargs["llm_model"] = args.llm_deployment

        md = MarkItDown(**kwargs)
        result = md.convert(args.input)

        if args.output:
            with open(args.output, "w", encoding="utf-8") as f:
                f.write(result.text_content)
            print(json.dumps({"success": True, "output_path": args.output}))
        else:
            print(json.dumps({"success": True, "content": result.text_content}))

    except Exception as e:
        print(json.dumps({"success": False, "error": str(e)}))
        sys.exit(1)


if __name__ == "__main__":
    main()
