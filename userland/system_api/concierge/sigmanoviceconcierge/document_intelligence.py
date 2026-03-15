# Generated method: SigmaNoviceConcierge.document_intelligence
import os

class SigmaNoviceConcierge:
    @staticmethod
    def document_intelligence(kernel, file_path, intent='Analyze'):
        """
            Novice-facing interface for PDF Forge.
            Converts 'Heavy' document tasks into simple intents.
            """
        if intent == 'Analyze':
            return kernel.process_document(file_path, 'Audit')
        elif intent == 'OCR':
            return kernel.process_document(file_path, 'OCR')
        return kernel.process_document(file_path, 'Analyze')