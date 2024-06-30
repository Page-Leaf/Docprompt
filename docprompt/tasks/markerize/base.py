from docprompt.tasks.base import AbstractPageTaskProvider, BasePageResult


class MarkerizeResult(BasePageResult):
    task_name = "markerize"
    raw_markdown: str


class BaseMarkerizeProvider(AbstractPageTaskProvider[None, MarkerizeResult]):
    class Meta:
        abstract = True