from .lrc_contents import LRC_CONTENTS
from translator.service import TranslationService
from translator.argos.provider import ArgosTranslator
from translator.argos.options import ArgosOptions
from translator.models.language import Language
import pytest

@pytest.mark.asyncio
async def test_argos_translator():
    lrc_contents = LRC_CONTENTS

    original_language = Language(
        name="Spanish",
        native_name="Español",
        code_2="es",
        code_3="spa",
    )
    to_language = Language(
        name="French",
        native_name="Français",
        code_2="fr",
        code_3="fra",
    )
    options = ArgosOptions(original_language=original_language)
    ArgosTranslator.install_packages(to_language, original_language)

    service = TranslationService({
        "argos": ArgosTranslator(),
    })
    translated_cues = await service.translate(
        provider_name="argos",
        cues=lrc_contents,
        to_language=to_language,
        options=options,
    )

    print(translated_cues)
