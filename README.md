# Docprompt

Docprompt is a library for Document AI. It aims to make enterprise-level document analysis easy thanks to the zero-shot capability of large language models while also providing a toolset for working with various document formats.

## Supercharged Document Analysis

* Common utilities for interacting with PDFs
  * PDF loading and serialization
  * PDF byte compression using Ghostscript :ghost:
  * Fast rasterization :fire: :rocket:
  * Page splitting, re-export with PDFium
* Support for most OCR providers with batched inference
  * Google :white_check_mark:
  * Azure Document Intelligence :red_circle:
  * Amazon Textract :red_circle:
  * Tesseract :red_circle:



[![pypi](https://img.shields.io/pypi/v/docprompt.svg)](https://pypi.org/project/docprompt/)
[![python](https://img.shields.io/pypi/pyversions/docprompt.svg)](https://pypi.org/project/docprompt/)
[![Build Status](https://github.com/psu3d0/docprompt/actions/workflows/dev.yml/badge.svg)](https://github.com/psu3d0/docprompt/actions/workflows/dev.yml)
[![codecov](https://codecov.io/gh/psu3d0/docprompt/branch/main/graphs/badge.svg)](https://codecov.io/github/psu3d0/docprompt)



Documents and large language models


* Documentation: <https://docprompt.dev>
* GitHub: <https://github.com/Page-Leaf/docprompt>
* PyPI: <https://pypi.org/project/docprompt/>
* Free software: Apache-2.0


## Features

* Representations for common document layout types - `TextBlock`, `BoundingBox`, etc
* Generic implementations of OCR providers

## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install Docprompt.

```bash
pip install docprompt
```

With an OCR provider

```bash
pip install "docprompt[google]
```


## Usage


### Simple Operations
```python
from docprompt import load_document

# Load a document
document = load_document("path/to/my.pdf")

# Rasterize a single page using Ghostscript
page_number = 5
rastered = document.rasterize_page(page_number, dpi=120)

# Split a pdf based on a page range
document_2 = document.split(start=125, stop=130)
```

### Performing OCR
```python
from docprompt import load_document, DocumentNode
from docprompt.tasks.ocr.gcp import GoogleOcrProvider

provider = GoogleOcrProvider.from_service_account_file(
  project_id=my_project_id,
  processor_id=my_processor_id,
  service_account_file=path_to_service_file
)

document = load_document("path/to/my.pdf")

# A container holds derived data for a document, like OCR or classification results
document_node = DocumentNode.from_document(document)

provider.process_document_node(document_node) # Caches results on the document_node

document_node[0].ocr_result # Access OCR results
```
