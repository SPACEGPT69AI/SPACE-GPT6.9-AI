from tenseal import CKKSContext, context

class SpaceDataEncryptor:
    def __init__(self, poly_modulus_degree=8192):
        self.ctx = CKKSContext(
            poly_modulus_degree=poly_modulus_degree,
            coeff_mod_bit_sizes=[60, 40, 40, 60]
        )
        self.ctx.generate_galois_keys()
        self.ctx.global_scale = 2 ** 40

    def encrypt_coordinates(self, data):
        vector = [data['lat'], data['lon'], data['alt']]
        encrypted = self.ctx.encrypt(vector)
        return {
            'ciphertext': encrypted.serialize(),
            'metadata': {
                'scale': self.ctx.global_scale,
                'modulus': self.ctx.coeff_modulus
            }
        }

    def decrypt_coordinates(self, ciphertext):
        encrypted = self.ctx.load(ciphertext['ciphertext'])
        return self.ctx.decrypt(encrypted)
