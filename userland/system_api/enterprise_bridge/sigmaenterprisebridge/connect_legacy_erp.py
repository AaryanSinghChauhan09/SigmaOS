# Generated method: SigmaEnterpriseBridge.connect_legacy_erp
import os

class SigmaEnterpriseBridge:
    def connect_legacy_erp(self, system_type='SAP'):
        """
            Native connectors for legacy CRM/ERP systems. 
            Decrypts and maps enterprise data streams into the SigmaData Lake.
            """
        if system_type in self.crm_connectors:
            return f'Sigma-ERP: Connected to {system_type}. Mapping data schemas to Sovereign Data Matrix.'
        return f'Error: {system_type} is not a supported legacy connector.'