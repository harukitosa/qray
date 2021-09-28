import numpy as np

class Qubits(object):
	def __init__(self, n_bits):
		self.n_bits = n_bits
		self.n_states = 2**n_bits
		self._amp = np.zeros(self.n_states, dtype=np.complex)
		self._amp[0] = 1

	def set_bits(self, bits):
		idx = sum(bit<<i for i, bit in enumerate(bits[::-1]))
		self._amp = np.zeros(self.n_states, dtype=np.complex)
		self._amp[idx] = 1.0
		return self


	def measure(self):
		print(self._amp)
		p = np.abs(self._amp)**2
		print("p:{}".format(p))
		idx = np.random.choice(range(len(self._amp)), p=p)
		print("idx:{}".format(idx))
		bits = [idx>>i & 1 for i in range(self.n_bits)]
		return bits[::-1]

	def apply(self, *operators):
		amp = self._amp
		for op in operators:
			amp = op(amp)
		applied = self.__class__(self.n_bits)
		applied._amp = amp
		return applied

	def __str__(self):
		return " + ".join(
			("{}|{:0" + str(self.n_bits) + "b}>").format(amp, i)
			for i, amp in enumerate(self._amp) if amp
			)

# Tests
# qubits = Qubits(5)
# # qubits.set_bits([0, 0, 0, 0, 1])
# print(qubits.measure())

import abc
import scipy.sparse as sp

class Operator(object):
	__metaclass__ = abc.ABCMeta
	def __init__(self, n_bits):
		self.n_bits = n_bits
		
	@abc.abstractproperty
	def matrix(self):
		pass
	
	def __call__(self, amp):
		return self.matrix.dot(amp)
		
	def __str__(self):
		return str(self._matrix.todense())

class X(Operator):
    def __init__(self, n_bits, target):
        super(X, self).__init__(n_bits)
        self.target = target

        self._matrix = sp.dok_matrix((2**n_bits, 2**n_bits))
        for upper in range(2**target):  # target より上位ビットをすべてなめる
            for lower in range(2**(n_bits - 1 - target)):  # target より下位のビットを全てなめる
                idx0 = upper*2**(n_bits - target) + lower  # target = 0 のインデックス
                idx1 = idx0 + 2**(n_bits - 1 - target)  # target = 1 のインデックス
                self._matrix[idx0, idx1] = 1
                self._matrix[idx1, idx0] = 1

    @property
    def matrix(self):
        return self._matrix

class H(Operator):
    def __init__(self, n_bits, target):
        super(H, self).__init__(n_bits)
        self.target = target

        self._matrix = sp.dok_matrix((2**n_bits, 2**n_bits))
        for upper in range(2**target):
            for lower in range(2**(n_bits - 1 - target)):
                idx0 = upper*2**(n_bits - target) + lower
                idx1 = idx0 + 2**(n_bits - 1 - target)
                self._matrix[idx0, idx0] = 1/np.sqrt(2)
                self._matrix[idx0, idx1] = 1/np.sqrt(2)
                self._matrix[idx1, idx0] = 1/np.sqrt(2)
                self._matrix[idx1, idx1] = -1/np.sqrt(2)

    @property
    def matrix(self):
        return self._matrix

h = H(1, 0)
print(h)

qubits = Qubits(2)
print('before: {}'.format(qubits))
print('after: {}'.format(qubits.apply(H(2, 1))))
print('after: {}'.format(qubits.apply(H(2, 1))))

print('m:{}'.format(qubits.apply(H(2, 0)).apply(H(2, 1)).measure()))