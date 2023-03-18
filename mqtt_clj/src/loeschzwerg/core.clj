(ns loeschzwerg.core
  (:require
   #_[clojure.core.async :refer [go timeout alts!]]
   [clojure.tools.logging :as log]
   [clojurewerkz.machine-head.client :as mqtt]))

(def connection (mqtt/connect "tcp://127.0.0.1"))

(defn start-consumer []
  (mqtt/subscribe connection {"ping/#" 1}
                  (fn [t m p]
                    (log/infof "[consumer/%s] %s %s" t (String. p "UTF-8") m))))

(defn start-producer []
  (let [topic    (str "ping/" (random-uuid))]
    (log/infof "[producer/%s] Producing..." topic)
    (loop [nonce 0]
      (mqtt/publish connection topic (str "ping:" nonce))
      (log/infof "[producer/%s] ping:%s" topic nonce)
      (Thread/sleep 100)
      (recur (inc nonce)))))


(defn -main [_]
  (log/info "[main] Hello MQTT")
  (start-consumer)
  (dotimes [_ 10]
    (future (start-producer)))
  (Thread/sleep 1000)
  (mqtt/disconnect-and-close connection)
  (log/warn "[main] Finishing...")
  0)
