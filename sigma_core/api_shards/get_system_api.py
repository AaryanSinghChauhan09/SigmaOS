


def get_system_api(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.api_shards.get_system_api')
    return getattr(mod, 'get_system_api')(*args, **kwargs)