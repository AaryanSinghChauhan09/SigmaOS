from userland.system_api.ai_integration.omni_prompt_distributor_shards.iaidistributor._base import IAIDistributor
from userland.system_api.ai_integration.omni_prompt_distributor_shards.sovereignomniprompt._base import SovereignOmniPrompt


def get_omni_prompt(*args, **kwargs):
    import importlib
    mod = importlib.import_module('userland.system_api.ai_integration.omni_prompt_distributor_shards.get_omni_prompt')
    return getattr(mod, 'get_omni_prompt')(*args, **kwargs)