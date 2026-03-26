


def process_file(*args, **kwargs):
    import importlib
    mod = importlib.import_module('.sigma_refactor_v12_shards.process_file')
    return getattr(mod, 'process_file')(*args, **kwargs)