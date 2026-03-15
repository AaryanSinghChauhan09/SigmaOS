# Generated method: SigmaModuleLoader.load_modules_parallel
import importlib
from .interfaces import ISigmaModule, ISigmaService

class SigmaModuleLoader:
    def load_modules_parallel(self, module_list: list):
        """
            USP: Concurrent Apex Hydration. 
            Loads multiple kernel/ecosystem shards in parallel via ThreadPool.
            """
        from concurrent.futures import ThreadPoolExecutor
        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = []
            for m_file, c_name, r_key in module_list:
                futures.append(executor.submit(self.load_module, m_file, c_name, r_key))
            for future in futures:
                try:
                    future.result()
                except:
                    pass
        return True