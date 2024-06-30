import importlib
from typing import Any, Literal


def attempt_import(name: str):
    """
    Attempts to import a module or class by name
    """
    package, obj = name.rsplit(".", 1)

    try:
        module = importlib.import_module(package)
    except ImportError:
        return None

    return getattr(module, obj, None)


SupportedModels = Literal["openai", "openai_async", "langchain"]


def validate_language_model(model: Any):
    langchain_chat_klass = attempt_import(
        "langchain_core.language_models.chat_models.BaseChatModel"
    )

    if langchain_chat_klass and isinstance(model, langchain_chat_klass):
        return "langchain"

    openai_klass = attempt_import("openai.OpenAI")

    if openai_klass and isinstance(model, openai_klass):
        return "openai"

    openai_async_klass = attempt_import("openai.OpenAIAsync")

    if openai_async_klass and isinstance(model, openai_async_klass):
        return "openai_async"

    raise ValueError(
        f"Model must be one of langchain_core.language_models.chat_models.BaseChatModel or openai.OpenAI. Got {type(model)}"
    )