// 外部からPromiseを制御するための関数.
// TODO: そのうちモダンで直感的な代替手段に変更する
export function createExternalPromise() {
  let resolve: (value: void | PromiseLike<void>) => void;
  let reject: (reason?: any) => void;

  const promisify = new Promise<void>((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });

  return {
    promisify,
    resolve: resolve!,
    reject: reject!,
  };
}
