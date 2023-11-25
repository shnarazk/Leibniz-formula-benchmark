// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

let beg = DispatchTime.now()
let pairs: Int = 100_000_000
let sum: Float = (0...pairs).reduce(0) { (result, index) in
  let k = Float(index) * 4.0
  return result + 8.0 / ((k + 1) * (k + 3))
}
let end = DispatchTime.now()
let duration = Double(end.uptimeNanoseconds - beg.uptimeNanoseconds) / 1_000_000_000
print("pi = \(sum) by \(pairs) pairs in \(duration) sec")
