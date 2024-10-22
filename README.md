# RUST html-to-pdf generator

```shell
curl -X POST http://localhost:3030/create_pdf \
-H "Content-Type: application/json" \
-d '{
    "doctor": {
      "fio": "Тест Солтанов АС",
      "post": "Зав. хирургического отделения",
      "speciality": "Врач-хирург"
    },
    "patient": {
      "cardno": "АБВ-ГД123:69es",
      "snils": "123-123-123 12",
      "fio": "Тест Солтанов АС",
      "date_born": "01.01.2000",
      "age": "24 года",
      "treatments": "Жалоб нет",
      "anamnes_morbi": "В пределах возрастной нормы",
      "objective_data": "В пределах возрастной нормы",
      "ecog": 0,
      "karnovsky_scale": 0,
      "clinical_ds": "Здоров",
      "mkb10_code": "H00.1",
      "recommends": "Пейте пиво пенное!"
    }
}'
```