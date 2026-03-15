


def sanitize(*args, **kwargs):
    import importlib
    mod = importlib.import_module('.sigma_refactor_v12_shards.sanitize')
    return getattr(mod, 'sanitize')(*args, **kwargs)