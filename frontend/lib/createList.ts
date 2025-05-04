type DataTransferGetter = () => DataTransfer;

let getDataTransfer: DataTransferGetter = (): DataTransfer => {
  return new DataTransfer();
};

// Fallback for environments where DataTransfer isn't available
try {
  getDataTransfer();
} catch {
  getDataTransfer = (): DataTransfer => {
    return new ClipboardEvent("").clipboardData as DataTransfer;
  };
}

const createFileList = (...files: File[]): FileList => {
  const dataTransfer = getDataTransfer();

  files.forEach((file) => {
    if (!(file instanceof File)) {
      throw new TypeError("Expected File objects");
    }
    dataTransfer.items.add(file);
  });

  return dataTransfer.files;
};

export default createFileList;
