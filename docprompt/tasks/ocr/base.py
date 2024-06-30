from typing import TYPE_CHECKING, Dict

from typing_extensions import override

from docprompt.schema.pipeline import DocumentNode
from docprompt.tasks.base import AbstractPageTaskProvider
from docprompt.tasks.ocr.result import OcrPageResult

if TYPE_CHECKING:
    from docprompt.schema.pipeline import DocumentNode


TInput = str


class BaseOCRProvider(AbstractPageTaskProvider[None, OcrPageResult]):
    def _populate_ocr_results(
        self, document_node: "DocumentNode", results: Dict[int, OcrPageResult]
    ) -> None:
        for page_number, result in results.items():
            document_node.page_nodes[page_number - 1].ocr_results.results[self.name] = (
                result
            )

    @override
    def process_document_node(
        self,
        document_node: DocumentNode,
        task_input: TInput,
        start: int | None = None,
        stop: int | None = None,
        contribute_to_document: bool = True,
        **kwargs,
    ) -> Dict[int, OcrPageResult]:
        base_result = super().process_document_node(
            document_node, task_input, start, stop, contribute_to_document, **kwargs
        )

        # For OCR, we also need to populate the ocr_results for powered search
        self._populate_ocr_results(document_node, base_result)

        return base_result